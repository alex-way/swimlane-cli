use crate::users::UserGroupSelection;
use crate::{BaseEntity, SwimlaneClient, SwimlaneClientError};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub users: Vec<BaseEntity>,
    pub groups: Vec<BaseEntity>,
    pub roles: Vec<BaseEntity>,
    #[serde(rename = "createdDate")]
    pub created_date: String, // todo: convert to date
    #[serde(rename = "modifiedDate")]
    pub modified_date: String, // todo: convert to date
    pub disabled: bool,
    #[serde(rename = "activeDirectoryGuids")]
    pub active_directory_guids: Option<Vec<String>>,
    #[serde(rename = "createdByUser")]
    pub created_by_user: UserGroupSelection,
    #[serde(rename = "modifiedByUser")]
    pub modified_by_user: UserGroupSelection,
}

impl SwimlaneClient {
    pub async fn get_groups(&self) -> Result<Vec<Group>, SwimlaneClientError> {
        let url = format!("{}/api/groups", self.base_url);

        #[derive(Deserialize)]
        struct GroupResponse {
            items: Vec<Group>,
        }

        // todo: recusively loop through all groups until there's no more groups
        let groups: GroupResponse = self.http_client.get(url).send().await?.json().await?;
        Ok(groups.items)
    }

    pub async fn create_group(&self, group: &Group) -> Result<Group, SwimlaneClientError> {
        let url = format!("{}/api/groups", self.base_url);
        let group: Group = self
            .http_client
            .post(url)
            .json(group)
            .send()
            .await?
            .json()
            .await?;
        Ok(group)
    }

    pub async fn update_group(&self, group: &Group) -> Result<Group, SwimlaneClientError> {
        let url = format!("{}/api/groups/{}", self.base_url, group.id);
        let group: Group = self
            .http_client
            .put(url)
            .json(group)
            .send()
            .await?
            .json()
            .await?;
        Ok(group)
    }
}