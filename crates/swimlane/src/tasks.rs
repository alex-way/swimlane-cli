use crate::apps::LightApplication;
use crate::{SwimlaneClient, SwimlaneClientError};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::create_dir;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskAction {
    pub readonly: bool,
    #[serde(rename = "type")]
    pub action_type: String,
    pub descriptor: ActionDescriptor,
    pub script: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightTask {
    pub id: String,
    pub name: String,
    pub disabled: bool,
    #[serde(rename = "applicationId")]
    pub application_id: Option<String>,
    #[serde(rename = "actionType")]
    pub action_type: String,
}

impl SwimlaneClient {
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

    // todo: move to the CLI crate
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

    // todo: move to the CLI crate
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

    // todo: move to the CLI crate
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

    // todo: move to the CLI crate
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

    // todo: move this to the migrator crate
    pub async fn get_task_hashmap(&self) -> Result<HashMap<String, String>, SwimlaneClientError> {
        let tasks = self.get_tasks_light().await?;

        let mut hashmap = HashMap::new();

        for task in tasks {
            hashmap.insert(task.name.clone(), task.id.clone());
            hashmap.insert(task.id.clone(), task.name.clone());
        }

        Ok(hashmap)
    }
}
