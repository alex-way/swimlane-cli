use crate::{SwimlaneClient, SwimlaneClientError};

use serde::{Deserialize, Serialize};

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
}
