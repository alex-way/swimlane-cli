pub mod error;
mod util;

use error::{SwimlaneClientError, UploadRequirementsError};
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::fs::create_dir;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};
use util::file_path_to_hashmap;

#[derive(Clone)]
pub struct SwimlaneClient {
    http_client: Client,
    base_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub disabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionDescriptor {
    pub id: String,
    pub name: String,
    pub disabled: bool,
    pub description: Option<String>,
    #[serde(rename = "actionType")]
    pub action_type: String,
    #[serde(rename = "actionId")]
    pub action_id: Option<String>,
    pub family: Option<String>,
    #[serde(rename = "base64Image")]
    pub base64_image: String,
    #[serde(rename = "assetDependencyType")]
    pub asset_dependency_type: Option<String>,
    #[serde(rename = "assetDependencyVersion")]
    pub asset_dependency_version: Option<String>,
    #[serde(rename = "pythonVersion")]
    pub python_version: Option<String>,
    #[serde(rename = "scriptFile")]
    pub script_file: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskAction {
    pub readonly: bool,
    #[serde(rename = "type")]
    pub action_type: String,
    pub descriptor: ActionDescriptor,
    pub script: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub valid: bool,
    pub disabled: bool,
    #[serde(rename = "applicationId")]
    pub application_id: Option<String>,
    pub action: TaskAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightTask {
    pub id: String,
    pub name: String,
    pub disabled: bool,
    #[serde(rename = "applicationId")]
    pub application_id: Option<String>,
    #[serde(rename = "actionType")]
    pub action_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightApplication {
    pub id: String,
    pub name: String,
    pub acronym: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn new(base_url: String, pat: String) -> Self {
        assert!(
            base_url.starts_with("https://"),
            "Invalid base url. Must start with https://"
        );

        let mut headers = HeaderMap::new();
        headers.insert("Private-Token", pat.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        let http_client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Could not build http client");

        Self {
            http_client,
            base_url,
        }
    }

    /// Pings the health endpoint and asserts a successful response.
    pub async fn health_ping(&self) -> Result<(), SwimlaneClientError> {
        let url = format!("{}/api/health/ping", self.base_url);
        let response = self.http_client.get(url).send().await?;
        response.error_for_status()?;
        println!("Health ping successful");
        Ok(())
    }

    /// Gets all users.
    pub async fn get_users(&self) -> Result<Vec<User>, SwimlaneClientError> {
        let url = format!("{}/api/users", self.base_url);
        let users: Vec<User> = self.http_client.get(url).send().await?.json().await?;
        Ok(users)
    }

    /// Gets the tasks (light model).
    pub async fn get_tasks_light(&self) -> Result<Vec<LightTask>, SwimlaneClientError> {
        let url = format!("{}/api/task/light", self.base_url);
        let tasks: Vec<LightTask> = self.http_client.get(url).send().await?.json().await?;
        Ok(tasks)
    }

    /// Gets common tasks.
    pub async fn get_common_tasks(&self) -> Result<Vec<Task>, SwimlaneClientError> {
        let url = format!("{}/api/task/common", self.base_url);
        let tasks: Vec<Task> = self.http_client.get(url).send().await?.json().await?;
        Ok(tasks)
    }

    /// Gets a task by id.
    pub async fn get_task(&self, task_id: &str) -> Result<Task, SwimlaneClientError> {
        let url = format!("{}/api/task/{}", self.base_url, task_id);
        let task: Task = self.http_client.get(url).send().await?.json().await?;
        Ok(task)
    }

    pub async fn get_applications_light(
        &self,
    ) -> Result<Vec<LightApplication>, SwimlaneClientError> {
        let url = format!("{}/api/app/light", self.base_url);
        let applications: Vec<LightApplication> =
            self.http_client.get(url).send().await?.json().await?;
        Ok(applications)
    }

    pub async fn get_tasks_for_application(
        &self,
        application_id: &str,
    ) -> Result<Vec<Task>, SwimlaneClientError> {
        let url = format!("{}/api/task?parentId={}", self.base_url, application_id);
        let tasks: Vec<Task> = self.http_client.get(url).send().await?.json().await?;
        Ok(tasks)
    }

    pub async fn download_tasks_for_application(
        &self,
        application: &LightApplication,
        path: &impl AsRef<Path>,
    ) -> Result<(), SwimlaneClientError> {
        println!("Downloading tasks for application: '{}'", application.name);
        let tasks = self.get_tasks_for_application(&application.id).await?;
        let folder = path.as_ref().join(&application.name);

        let downloadable_tasks = tasks
            .into_iter()
            .filter(|t| t.action.script.is_some())
            .collect::<Vec<_>>();

        if !folder.exists() && !downloadable_tasks.is_empty() {
            create_dir(&folder)
                .unwrap_or_else(|_| panic!("Could not create folder: '{}'", folder.display()));
        }

        let mut handles = vec![];

        for task in downloadable_tasks {
            let folder = folder.clone();
            let client = self.clone();
            let handle = tokio::spawn(async move {
                println!("Downloading task: '{}'", task.name);
                client
                    .save_task(&task, &folder)
                    .await
                    .unwrap_or_else(|_| panic!("Could not save task: '{}'", task.name));
                println!("Downloaded task: '{}'", task.name);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        Ok(())
    }

    pub async fn download_common_tasks(
        &self,
        path: &impl AsRef<Path>,
    ) -> Result<(), SwimlaneClientError> {
        println!("Downloading common tasks");
        let tasks = self.get_common_tasks().await?;
        let folder = path.as_ref().join("common");

        let downloadable_tasks = tasks
            .into_iter()
            .filter(|t| t.action.script.is_some())
            .collect::<Vec<_>>();

        if !folder.exists() && !downloadable_tasks.is_empty() {
            create_dir(&folder)
                .unwrap_or_else(|_| panic!("Could not create folder: '{}'", folder.display()));
        }

        let mut handles = vec![];

        for task in downloadable_tasks {
            let folder = folder.clone();
            let client = self.clone();
            let handle = tokio::spawn(async move {
                println!("Downloading task: '{}'", task.name);
                client
                    .save_task(&task, &folder)
                    .await
                    .unwrap_or_else(|_| panic!("Could not save task: '{}'", task.name));
                println!("Downloaded task: '{}'", task.name);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        Ok(())
    }

    async fn save_task(&self, task: &Task, path: &impl AsRef<Path>) -> io::Result<()> {
        match &task.action.script {
            Some(script) => {
                let file_path = path.as_ref().join(format!("{}.py", task.name));
                let mut file = File::create(&file_path).await?;
                file.write_all(script.as_bytes()).await.unwrap_or_else(|_| {
                    panic!("Could not write to file: '{}'", &file_path.display())
                });
                Ok(())
            }
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Task '{}' has no script", task.name),
            )),
        }
    }

    /// Downloads all python tasks to the specified path in the format '{application_name}/{task_name}.py'
    pub async fn download_python_tasks(
        &self,
        path: &impl AsRef<Path>,
    ) -> Result<(), SwimlaneClientError> {
        // Create the path if it doesnt exist
        if !path.as_ref().exists() {
            std::fs::create_dir(path).unwrap_or_else(|_| {
                panic!("Could not create path: '{}'", &path.as_ref().display())
            });
        }

        let applications = self
            .get_applications_light()
            .await
            .expect("Could not get applications");

        let mut handles = vec![];

        for application in applications {
            // todo: remove requirement for cloning
            let sw = self.clone();
            let path = path.as_ref().to_path_buf();
            let handle = tokio::spawn(async move {
                sw.download_tasks_for_application(&application, &path).await
            });
            handles.push(handle);
        }

        let common_path = path.as_ref().to_path_buf();
        let sw = self.clone();
        handles.push(tokio::spawn(async move {
            sw.download_common_tasks(&common_path).await
        }));

        for handle in handles {
            handle.await.unwrap()?;
        }

        Ok(())
    }

    pub async fn get_installed_pip_packages(&self) -> Result<Vec<PipPackage>, SwimlaneClientError> {
        let url = format!("{}/api/pip/packages/3", self.base_url);
        let packages: Vec<PipPackage> = self.http_client.get(url).send().await?.json().await?;
        // todo: make name, version lowercase
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
