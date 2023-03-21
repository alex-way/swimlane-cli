use crate::tasks::Task;
use crate::{SwimlaneClient, SwimlaneClientError};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightApplication {
    pub id: String,
    pub name: String,
    pub acronym: String,
    pub description: Option<String>,
}

impl SwimlaneClient {
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
}
