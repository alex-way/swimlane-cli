use crate::{SwimlaneClient, SwimlaneClientError};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub disabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserGroupSelection {
    pub id: Option<String>,
    pub name: Option<String>,
}

impl SwimlaneClient {
    /// Gets all users.
    pub async fn get_users(&self) -> Result<Vec<User>, SwimlaneClientError> {
        let url = format!("{}/api/user", self.base_url);
        self.get_paginated_items::<User>(&url).await
    }
}
