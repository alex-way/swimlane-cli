use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct SwimlaneClient {
    http_client: Client,
    base_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    email: String,
    #[serde(rename = "userName")]
    user_name: String,
    id: String,
    disabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActionDescriptor {
    id: String,
    name: String,
    disabled: bool,
    description: Option<String>,
    #[serde(rename = "actionType")]
    action_type: String,
    #[serde(rename = "actionId")]
    action_id: Option<String>,
    family: Option<String>,
    #[serde(rename = "base64Image")]
    base64_image: String,
    #[serde(rename = "assetDependencyType")]
    asset_dependency_type: Option<String>,
    #[serde(rename = "assetDependencyVersion")]
    asset_dependency_version: Option<String>,
    #[serde(rename = "pythonVersion")]
    python_version: Option<String>,
    #[serde(rename = "scriptFile")]
    script_file: Option<String>,
    version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskAction {
    readonly: bool,
    #[serde(rename = "type")]
    action_type: String,
    descriptor: ActionDescriptor,
    script: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: String,
    name: String,
    description: Option<String>,
    valid: bool,
    disabled: bool,
    #[serde(rename = "applicationId")]
    application_id: Option<String>,
    action: TaskAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightTask {
    id: String,
    name: String,
    disabled: bool,
    #[serde(rename = "applicationId")]
    application_id: Option<String>,
    #[serde(rename = "actionType")]
    action_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightApplication {
    id: String,
    name: String,
    acronym: String,
    description: Option<String>,
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
        if !folder.exists() && tasks.iter().any(|t| t.action.script.is_some()) {
            std::fs::create_dir(&folder).expect("Could not create folder");
        }

        for task in tasks {
            self.save_task(&task, &folder);
        }

        Ok(())
    }

    fn save_task(&self, task: &Task, path: &impl AsRef<Path>) {
        match &task.action.script {
            Some(script) => {
                let file_path = path.as_ref().join(format!("{}.py", task.name));
                std::fs::write(file_path, script).expect("Could not write file");
            }
            None => {}
        }
    }

    /// Downloads all python tasks to the specified path in the format '{application_name}/{task_name}.py'
    pub async fn download_python_tasks(
        &self,
        path: &impl AsRef<Path>,
    ) -> Result<(), reqwest::Error> {
        let applications = self.get_applications_light().await?;

        for application in applications {
            self.download_tasks_for_application(&application, path)
                .await?;
        }

        Ok(())
    }
}
