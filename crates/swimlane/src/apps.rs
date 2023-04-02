use std::fmt::{Display, Formatter};

use crate::roles::PermissionMatrix;
use crate::tasks::Task;
use crate::users::UserGroupSelection;
use crate::{SwimlaneClient, SwimlaneClientError};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightApplication {
    pub id: String,
    pub name: String,
    pub acronym: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LayoutType {
    #[serde(rename = "field")]
    Field,
    #[serde(rename = "section")]
    Section,
    #[serde(rename = "htmlObject")]
    HtmlObject,
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "integration")]
    Integration,
    #[serde(rename = "tabs")]
    Tabs,
    #[serde(rename = "widget")]
    Widget,
    #[serde(rename = "orchestrationTask")]
    OrchestrationTask,
}

// todo: Convert Layout to an enum of different layout types. As the optional properties like html are only required
// by certain layout types.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layout {
    pub id: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    pub row: i32,
    pub col: i32,
    #[serde(rename = "sizex")]
    pub size_x: f32,
    #[serde(rename = "sizey")]
    pub size_y: f32,
    #[serde(rename = "layoutType")]
    pub layout_type: LayoutType,
    pub html: Option<String>,
    pub children: Option<Vec<Layout>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FieldType {
    None,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "numeric")]
    Numeric,
    #[serde(rename = "valuesList")]
    ValuesList,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "userGroup")]
    UserGroup,
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "tracking")]
    Tracking,
    #[serde(rename = "reference")]
    Reference,
    #[serde(rename = "comments")]
    Comments,
    #[serde(rename = "history")]
    History,
    #[serde(rename = "list")]
    List,
}

// todo: Turn field into separate field types, with the correct types for each field
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub id: String,
    pub name: String,
    pub key: String,
    #[serde(rename = "sourceAppletFieldId")]
    pub source_applet_field_id: Option<String>,
    #[serde(rename = "sourceAppletId")]
    pub source_applet_id: Option<String>,
    #[serde(rename = "fieldType")]
    pub field_type: FieldType,
    pub required: Option<bool>,
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    #[serde(rename = "supportsMultipleOutputMappings")]
    pub supports_multiple_output_mappings: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub acronym: String,
    pub description: Option<String>,
    #[serde(rename = "trackingFieldId")]
    pub tracking_field_id: String,
    pub layout: Vec<Layout>,
    pub fields: Vec<Field>,
    #[serde(rename = "maxTrackingId")]
    pub max_tracking_id: f32,
    pub workspaces: Vec<String>,
    #[serde(rename = "createWorkspace")]
    pub create_workspace: bool,
    #[serde(rename = "createdDate")]
    pub created_date: DateTime<Utc>,
    #[serde(rename = "createdByUser")]
    pub created_by_user: UserGroupSelection,
    #[serde(rename = "modifiedDate")]
    pub modified_date: DateTime<Utc>,
    #[serde(rename = "modifiedByUser")]
    pub modified_by_user: UserGroupSelection,
    #[serde(rename = "timeTrackingEnabled")]
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

    pub async fn get_applications(&self) -> Result<Vec<Application>, SwimlaneClientError> {
        let url = format!("{}/api/app", self.base_url);
        let applications: Vec<Application> = self.http_client.get(url).send().await?.json().await?;
        Ok(applications)
    }
}
