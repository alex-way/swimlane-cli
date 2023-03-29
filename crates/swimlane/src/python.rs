use crate::error::{SwimlaneClientError, UploadRequirementsError};
use crate::util::file_path_to_hashmap;
use crate::SwimlaneClient;

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipPackage {
    pub name: String,
    pub version: Option<String>,
    #[serde(rename = "pythonVersion")]
    pub python_version: String,
    pub id: Option<String>,
    pub requires: Option<Vec<String>>,
    pub license: Option<String>,
    #[serde(rename = "authorEmail")]
    pub author_email: Option<String>,
    pub author: Option<String>,
    #[serde(rename = "homePage")]
    pub home_page: Option<String>,
    pub summary: Option<String>,
    pub disabled: Option<bool>,
}

impl SwimlaneClient {
    pub async fn get_installed_pip_packages(&self) -> Result<Vec<PipPackage>, SwimlaneClientError> {
        let url = format!("{}/api/pip/packages/3", self.base_url);
        let packages: Vec<PipPackage> = self.http_client.get(url).send().await?.json().await?;
        let packages = packages
            .into_iter()
            .map(|mut p| {
                p.name = p.name.to_lowercase();
                p
            })
            .collect::<Vec<_>>();

        Ok(packages)
    }

    pub async fn upload_python_requirements(
        &self,
        file_path: &impl AsRef<Path>,
    ) -> Result<(), SwimlaneClientError> {
        let local_requirements =
            file_path_to_hashmap(file_path).unwrap_or_else(|error| match error {
                UploadRequirementsError::DuplicatePackage {
                    key,
                    line_number,
                    existing_value,
                    new_value,
                } => {
                    panic!(
                        "Duplicate package '{}' found on line {} with versions '{}' and '{}'",
                        key, line_number, existing_value, new_value
                    )
                }
                UploadRequirementsError::FileNotFound(err) => {
                    panic!("File not found: '{}'", err)
                }
                UploadRequirementsError::InvalidFormat { line_number, line } => {
                    panic!("Invalid package '{}' found on line {}", line, line_number)
                }
            });

        let already_installed_packages = self.get_installed_pip_packages().await?;

        let mut packages_to_install = vec![];
        let mut packages_to_update = vec![];

        for package in local_requirements {
            if already_installed_packages
                .iter()
                .any(|p| p.name == package.0 && p.version != Some(package.1.to_string()))
            {
                println!("Package '{}' needs to be updated", package.0);
                packages_to_update.push(package);
            } else if !already_installed_packages
                .iter()
                .any(|p| p.name == package.0 && p.version == Some(package.1.to_string()))
            {
                packages_to_install.push(package);
            } else {
                println!("Package '{}=={}' already installed", package.0, package.1);
            }
        }

        let mut handles = vec![];

        for (package, _) in packages_to_update.clone() {
            let sw = self.clone();
            let handle =
                tokio::spawn(async move { sw.uninstall_pip_package(package.as_str()).await });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap()?;
        }

        let mut handles = vec![];

        packages_to_install.append(&mut packages_to_update);

        for (package, version) in packages_to_install {
            let sw = self.clone();
            let handle = tokio::spawn(async move {
                sw.install_pip_package(package.as_str(), Some(version.as_str()))
                    .await
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap()?;
        }

        Ok(())
    }

    pub async fn install_pip_package(
        &self,
        name: &str,
        version: Option<&str>,
    ) -> Result<(), SwimlaneClientError> {
        let version_to_install: Option<String> = match version {
            Some(v) => {
                println!("Installing pip package '{}=={}'", name, v);
                Some(v.to_string())
            }
            None => {
                println!("Installing pip package '{}' @ latest", name);
                None
            }
        };

        let url = format!("{}/api/pip/packages/", self.base_url);
        let body = PipPackage {
            name: name.to_string(),
            version: version_to_install,
            python_version: "Python3".to_string(),
            id: None,
            author: None,
            license: None,
            disabled: None,
            requires: None,
            author_email: None,
            home_page: None,
            summary: None,
        };

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap_or_else(|_| panic!("Could not upload requirement: '{}'", name));
        if response.status() != 200 {
            panic!(
                "Could not upload requirement: '{}', status: '{}'",
                name,
                response.status()
            );
        }
        Ok(())
    }

    pub async fn uninstall_pip_package(&self, name: &str) -> Result<(), SwimlaneClientError> {
        println!("Uninstalling pip package '{}'", name);
        let url = format!("{}/api/pip/packages/{}/3", self.base_url, name);
        let response = self
            .http_client
            .delete(url)
            .send()
            .await
            .unwrap_or_else(|_| panic!("Could not uninstall requirement: '{}'", name));
        if response.status() != 200 && response.status() != 204 {
            panic!(
                "Could not uninstall requirement: '{}', status: '{}'",
                name,
                response.status()
            );
        }
        Ok(())
    }
}
