use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use crate::{
    apps::Application, error::SwimlaneClientError, roles::PermissionMatrix,
    users::UserGroupSelection, SwimlaneClient,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    #[serde(rename = "$type")]
    pub _type: String,
    pub id: String,
    pub uid: String,
    pub description: Option<String>,
    pub version: i32,
    pub name: String,
    pub disabled: bool,
    pub applications: Vec<String>,
    pub dashboards: Vec<String>,
    pub modified_by_user: Option<UserGroupSelection>,
    pub created_by_user: Option<UserGroupSelection>,
    pub modified_date: DateTime<Utc>,
    pub created_date: DateTime<Utc>,
    pub permissions: PermissionMatrix, // TODO : Ensure that we implement the permissions
}

impl Workspace {
    pub async fn convert_app_ids_to_names(
        &self,
        sc: SwimlaneClient,
    ) -> Result<Application, SwimlaneClientError> {
        for app_id in &self.applications {
            let app: Application = sc.get_application(app_id).await?;
            println!("App: {:?}", app.name);
        }
        todo!()
    }

    pub async fn convert_dashboard_ids_to_names(&self) {
        todo!()
    }
}

impl Display for Workspace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Workspace: {}", self.name)
    }
}

impl SwimlaneClient {
    /// Get all workspaces
    pub async fn get_workspaces(&self) -> Result<Vec<Workspace>, SwimlaneClientError> {
        let url: String = format!("{}/api/workspaces", self.base_url);
        let workspaces: Vec<Workspace> = self.http_client.get(url).send().await?.json().await?;
        Ok(workspaces)
    }
}
