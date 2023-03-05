use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::fs::create_dir;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

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

    /// Pings the health endpoint and asserts a 200 response.
    pub async fn health_ping(&self) -> Result<(), reqwest::Error> {
        let url = format!("{}/api/health/ping", self.base_url);
        let response = self.http_client.get(url).send().await?;
        assert_eq!(response.status(), 200);
        println!("Health ping successful");
        Ok(())
    }

    /// Gets all users.
    pub async fn get_users(&self) -> Result<Vec<User>, reqwest::Error> {
        let url = format!("{}/api/users", self.base_url);
        let users: Vec<User> = self.http_client.get(url).send().await?.json().await?;
        Ok(users)
    }

    /// Gets the tasks (light model).
    pub async fn get_tasks_light(&self) -> Result<Vec<LightTask>, reqwest::Error> {
        let url = format!("{}/api/task/light", self.base_url);
        let tasks: Vec<LightTask> = self.http_client.get(url).send().await?.json().await?;
        Ok(tasks)
    }

    /// Gets common tasks.
    pub async fn get_common_tasks(&self) -> Result<Vec<Task>, reqwest::Error> {
        let url = format!("{}/api/task/common", self.base_url);
        let tasks: Vec<Task> = self.http_client.get(url).send().await?.json().await?;
        Ok(tasks)
    }

    /// Gets a task by id.
    pub async fn get_task(&self, task_id: &str) -> Result<Task, reqwest::Error> {
        let url = format!("{}/api/task/{}", self.base_url, task_id);
        let task: Task = self.http_client.get(url).send().await?.json().await?;
        Ok(task)
    }

    pub async fn get_applications_light(&self) -> Result<Vec<LightApplication>, reqwest::Error> {
        let url = format!("{}/api/app/light", self.base_url);
        let applications: Vec<LightApplication> =
            self.http_client.get(url).send().await?.json().await?;
        Ok(applications)
    }

    pub async fn get_tasks_for_application(
        &self,
        application_id: &str,
    ) -> Result<Vec<Task>, reqwest::Error> {
        let url = format!("{}/api/task?parentId={}", self.base_url, application_id);
        let tasks: Vec<Task> = self.http_client.get(url).send().await?.json().await?;
        Ok(tasks)
    }

    pub async fn download_tasks_for_application(
        &self,
        application: &LightApplication,
        path: &impl AsRef<Path>,
    ) -> Result<(), reqwest::Error> {
        println!("Downloading tasks for application: '{}'", application.name);
        let tasks = self.get_tasks_for_application(&application.id).await?;
        let folder = path.as_ref().join(&application.name);

        let downloadable_tasks = tasks
            .into_iter()
            .filter(|t| t.action.script.is_some())
            .collect::<Vec<_>>();

        if !folder.exists() && downloadable_tasks.len() > 0 {
            create_dir(&folder)
                .expect(format!("Could not create folder: '{}'", folder.display()).as_str());
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
                    .expect(format!("Could not save task: '{}'", task.name).as_str());
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
    ) -> Result<(), reqwest::Error> {
        println!("Downloading common tasks");
        let tasks = self.get_common_tasks().await?;
        let folder = path.as_ref().join("common");

        let downloadable_tasks = tasks
            .into_iter()
            .filter(|t| t.action.script.is_some())
            .collect::<Vec<_>>();

        if !folder.exists() && downloadable_tasks.len() > 0 {
            create_dir(&folder)
                .expect(format!("Could not create folder: '{}'", folder.display()).as_str());
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
                    .expect(format!("Could not save task: '{}'", task.name).as_str());
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
                file.write_all(script.as_bytes()).await.expect(
                    format!("Could not write to file: '{}'", &file_path.display()).as_str(),
                );
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
    ) -> Result<(), reqwest::Error> {
        // Create the path if it doesnt exist
        if !path.as_ref().exists() {
            std::fs::create_dir(&path)
                .expect(format!("Could not create path: '{}'", &path.as_ref().display()).as_str());
        }

        let applications = self
            .get_applications_light()
            .await
            .expect("Could not get applications");

        let mut handles = vec![];

        for application in applications {
            // todo: remove requirement for cloning
            let sw = self.clone();
            let path = path.clone().as_ref().to_path_buf();
            let handle = tokio::spawn(async move {
                sw.download_tasks_for_application(&application, &path).await
            });
            handles.push(handle);
        }

        let common_path = path.clone().as_ref().to_path_buf();
        let sw = self.clone();
        handles.push(tokio::spawn(async move {
            sw.download_common_tasks(&common_path).await
        }));

        for handle in handles {
            handle.await.unwrap().unwrap();
        }

        Ok(())
    }
}
