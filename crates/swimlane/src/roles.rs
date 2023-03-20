use std::collections::HashMap;

use crate::users::UserGroupSelection;
use crate::util::PagedResponse;
use crate::{BaseEntity, SwimlaneClient, SwimlaneClientError};
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PermissionType {
    Global,
    Application,
    Report,
    Dashboard,
    Workspace,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionMatrix {
    #[serde(rename = "$type")]
    _type: String,
    #[serde(flatten)]
    pub permissions: HashMap<String, Permission>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Access {
    #[serde(rename = "$type")]
    _type: String,
    #[serde(flatten)]
    pub permissions: HashMap<String, u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Permission {
    #[serde(rename = "$type")]
    _type: String,
    #[serde(rename = "type")]
    pub type_: PermissionType,
    pub id: String,
    pub name: String,
    pub access: u16,
    pub fields: Access,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub disabled: bool,
    pub description: Option<String>,
    pub permissions: PermissionMatrix,
    #[serde(rename = "createdDate")]
    pub created_date: DateTime<Utc>,
    #[serde(rename = "modifiedDate")]
    pub modified_date: DateTime<Utc>,
    #[serde(rename = "createdByUser")]
    pub created_by_user: Option<UserGroupSelection>,
    #[serde(rename = "modifiedByUser")]
    pub modified_by_user: Option<UserGroupSelection>,
    pub groups: Vec<BaseEntity>,
    pub roles: Option<Vec<BaseEntity>>,
    pub users: Vec<BaseEntity>,
}

impl SwimlaneClient {
    pub async fn get_roles(&self) -> Result<Vec<Role>, SwimlaneClientError> {
        let url = format!("{}/api/roles/", self.base_url);
        let response = self.http_client.get(&url).send().await?;

        // todo: recusively loop through all users until there's no more users
        let roles: PagedResponse<Role> = response.json().await?;
        Ok(roles.items)
    }
}
