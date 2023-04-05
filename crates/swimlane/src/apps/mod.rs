pub mod fields;
pub mod layout;

use std::fmt::{Display, Formatter};

use crate::roles::PermissionMatrix;
use crate::tasks::Task;
use crate::users::UserGroupSelection;
use crate::{SwimlaneClient, SwimlaneClientError};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::fields::Field;
use self::layout::Layout;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct LightApplication {
    #[serde(rename = "$type")]
    pub _type: String,
    pub id: String,
    pub name: String,
    pub acronym: String,
    pub description: Option<String>,
    pub created_date: DateTime<Utc>,
    pub created_by_user: UserGroupSelection,
    pub modified_date: DateTime<Utc>,
    pub modified_by_user: UserGroupSelection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    #[serde(rename = "$type")]
    pub _type: String,
    pub id: String,
    pub name: String,
    pub acronym: String,
    pub description: Option<String>,
    pub tracking_field_id: String,
    pub layout: Vec<Layout>,
    pub fields: Vec<Field>,
    pub max_tracking_id: f32,
    pub workspaces: Vec<String>,
    pub create_workspace: bool,
    pub created_date: DateTime<Utc>,
    pub created_by_user: UserGroupSelection,
    pub modified_date: DateTime<Utc>,
    pub modified_by_user: UserGroupSelection,
    pub time_tracking_enabled: bool,
    pub permissions: PermissionMatrix,
    pub uid: String,
    pub version: i32,
    pub disabled: bool,
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application: {}", self.name)
    }
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

    pub async fn get_application(
        &self,
        application_id: &str,
    ) -> Result<Application, SwimlaneClientError> {
        let url = format!("{}/api/app/{}", self.base_url, application_id);
        let app: Application = self.http_client.get(url).send().await?.json().await?;

        Ok(app)
    }

    pub async fn get_applications(&self) -> Result<Vec<Application>, SwimlaneClientError> {
        let url = format!("{}/api/app", self.base_url);
        let apps: Vec<Application> = self.http_client.get(url).send().await?.json().await?;

        Ok(apps)
    }

    pub async fn create_application(
        &self,
        app: &Application,
    ) -> Result<Application, SwimlaneClientError> {
        let url = format!("{}/api/app", self.base_url);
        let app: Application = self
            .http_client
            .post(url)
            .json(app)
            .send()
            .await?
            .json()
            .await?;

        Ok(app)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_application_deserializes() {
        let json = r#"{
              "$type": "Core.Models.Application.ApplicationViewModel, Core",
              "id": "aMrGgbzJ9aVicl9DL",
              "name": "Alert & Incident Management",
              "acronym": "SAIM",
              "description": "",
              "createdDate": "2022-07-27T16:08:45.045Z",
              "createdByUser": {
                "$type": "Core.Models.Utilities.UserGroupSelection, Core"
              },
              "modifiedDate": "2023-03-21T14:41:52.743Z",
              "modifiedByUser": {
                "$type": "Core.Models.Utilities.UserGroupSelection, Core",
                "id": "a4UAS4FfjxrOAfrue",
                "name": "James Bond"
              }
            }"#;

        let app: LightApplication = serde_json::from_str(json).unwrap();
        assert_eq!(app.id, "aMrGgbzJ9aVicl9DL");
        assert_eq!(app.name, "Alert & Incident Management");
        assert_eq!(app.acronym, "SAIM");
        assert_eq!(app.description, Some("".to_string()));
    }

    #[test]
    fn test_saim_application_deserializes() {
        let json = include_str!("json/saim.json");

        let app: Application = serde_json::from_str(json).unwrap();
        assert_eq!(app.id, "aMrGgbzJ9aVicl9DL");
        assert_eq!(app.name, "Alert & Incident Management");
        assert_eq!(app.acronym, "SAIM");
        assert_eq!(app.description, Some("".to_string()));
    }

    #[test]
    fn test_applications_deserializes() {
        let json = include_str!("json/apps.json");

        let apps: Vec<Application> = serde_json::from_str(json).unwrap();

        println!("{:#?}", apps);
    }
}
