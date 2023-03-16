use crate::users::UserGroupSelection;
use crate::{BaseEntity, SwimlaneClient, SwimlaneClientError};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub disabled: bool,
    pub description: Option<String>,
    // permissions: Vec<String>, // todo: handle permissions matrix
    #[serde(rename = "createdDate")]
    pub created_date: String, // todo: convert to DateTime
    #[serde(rename = "modifiedDate")]
    pub modified_date: String, // todo: convert to DateTime
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
        #[derive(Deserialize)]
        struct RoleResponse {
            items: Vec<Role>,
        }
        let roles: RoleResponse = response.json().await?;
        Ok(roles.items)
    }
}
