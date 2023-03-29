use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use crate::{BaseEntity, SwimlaneClient, SwimlaneClientError};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SessionTimeoutType {
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "hours")]
    Hours,
}

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
    pub favorites: HashMap<String, String>,
    #[serde(rename = "middleInitial")]
    pub middle_initial: Option<String>,
    pub active: bool,
    #[serde(rename = "lastPasswordChangedDate")]
    pub last_password_changed_date: DateTime<Utc>,
    #[serde(rename = "passwordResetRequired")]
    pub password_reset_required: bool,
    #[serde(rename = "sessionTimeoutType")]
    pub session_timeout_type: SessionTimeoutType,
    #[serde(rename = "primaryGroup")]
    pub primary_group: Option<UserGroupSelection>,
    pub groups: Vec<BaseEntity>,
    pub roles: Vec<BaseEntity>,
    #[serde(rename = "createdDate")]
    pub created_date: DateTime<Utc>,
    #[serde(rename = "modifiedDate")]
    pub modified_date: DateTime<Utc>,
    #[serde(rename = "createdByUser")]
    pub created_by_user: Option<UserGroupSelection>,
    #[serde(rename = "modifiedByUser")]
    pub modified_by_user: Option<UserGroupSelection>,
    #[serde(rename = "passwordComplexityScore")]
    pub password_complexity_score: Option<i32>,
    #[serde(rename = "isSystemUser")]
    pub is_system_user: bool,
    #[serde(rename = "timeZoneId")]
    pub timezone_id: String,
    #[serde(rename = "isOTPVerified")]
    pub is_otp_verified: bool,
    #[serde(rename = "isOtpUser")]
    pub is_otp_user: bool,
    #[serde(rename = "isOtpEnforced")]
    pub is_otp_enforced: bool,
    #[serde(rename = "isOtpExempted")]
    pub is_otp_exempted: bool,
    #[serde(rename = "isLdapUser")]
    pub is_ldap_user: bool,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "currentFailedLogInAttempts")]
    pub current_failed_log_in_attempts: i32,
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    #[serde(rename = "lastLogin")]
    pub last_login: Option<DateTime<Utc>>,
    #[serde(rename = "activeDirectoryGuid")]
    pub active_directory_guid: Option<String>,
    pub domain: Option<String>,
    pub avatar: Option<String>,
    #[serde(rename = "defaultWorkspaceId")]
    pub default_workspace_id: Option<String>,
    #[serde(rename = "defaultDashboardId")]
    pub default_dashboard_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCreationRequest {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub notify: bool,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "middleInitial")]
    pub middle_initial: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub password: String,
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,
    pub email: String,
    pub groups: Vec<BaseEntity>,
    pub roles: Vec<BaseEntity>,
}

impl From<User> for UserCreationRequest {
    fn from(item: User) -> Self {
        UserCreationRequest {
            user_name: item.user_name,
            notify: false,
            first_name: item.first_name,
            last_name: item.last_name,
            middle_initial: item.middle_initial,
            display_name: item.display_name,
            password: "".to_string(),
            confirm_password: "".to_string(),
            email: item.email,
            groups: item.groups,
            roles: item.roles,
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: {}", self.user_name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserGroupSelection {
    pub id: Option<String>,
    pub name: Option<String>,
}

impl PartialEq for UserGroupSelection {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Display for UserGroupSelection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => write!(f, "UserGroupSelection: {}", name),
            None => write!(f, "UserGroupSelection: None"),
        }
    }
}

impl SwimlaneClient {
    /// Gets all users.
    pub async fn get_users(&self) -> Result<Vec<User>, SwimlaneClientError> {
        let url = format!("{}/api/user", self.base_url);
        self.get_paginated_items::<User>(&url).await
    }

    pub async fn create_user(
        &self,
        user: &UserCreationRequest,
    ) -> Result<User, SwimlaneClientError> {
        let url = format!("{}/api/user", self.base_url);
        let user: User = self
            .http_client
            .post(url)
            .json(user)
            .send()
            .await?
            .json()
            .await?;
        Ok(user)
    }

    pub async fn update_user(&self, user: &User) -> Result<User, SwimlaneClientError> {
        let url = format!("{}/api/user/{}", self.base_url, user.id);
        let user: User = self
            .http_client
            .put(url)
            .json(user)
            .send()
            .await?
            .json()
            .await?;
        Ok(user)
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<(), SwimlaneClientError> {
        let url = format!("{}/api/user/{}", self.base_url, user_id);
        self.http_client.delete(url).send().await?;
        Ok(())
    }
}
