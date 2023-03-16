use std::collections::HashMap;

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

    // todo: move this to the migrator crate
    pub async fn get_application_hashmap(
        &self,
    ) -> Result<HashMap<String, String>, SwimlaneClientError> {
        let applications = self.get_applications_light().await?;

        let mut hashmap = HashMap::new();

        for application in applications {
            hashmap.insert(application.name.clone(), application.id.clone());
            hashmap.insert(application.id.clone(), application.name.clone());
        }

        Ok(hashmap)
    }
}
