mod adapt;
pub mod equality;
pub mod groups;
pub mod roles;
pub mod users;
pub mod util;

use equality::LooksLike;
use swimlane::SwimlaneClient;
use thiserror::Error;

pub struct SwimlaneMigrator {
    pub from: SwimlaneClient,
    pub to: SwimlaneClient,
    pub dry_run: bool,
}

#[derive(Error, Debug)]
pub enum SwimlaneMigratorNewError {
    #[error("The source and destination Swimlane servers are the same. Please specify different servers.")]
    SourceAndDestinationAreIdentical,
}

#[derive(Error, Debug)]
pub enum SwimlaneMigratorError {
    #[error("Swimlane error")]
    SwimlaneError(#[from] swimlane::error::SwimlaneClientError),
    #[error("Group not found: {group_name}")]
    MissingGroup { group_name: String },
    #[error("Role not found: {role_name}")]
    MissingRole { role_name: String },
    #[error("User not found: {user_name}")]
    MissingUser { user_name: String },
}

pub enum MigrationPlan<T: LooksLike> {
    Create {
        source_resource: T,
    },
    Update {
        source_resource: T,
        destination_resource: T,
    },
    Delete {
        // Currently it's not possible for there to be a delete action as logically it's not possible to delete something
        // that doesn't exist. However, this may change in the future. Hence leaving this here.
        destination_resource: T,
    },
}

// todo: turn the differences into a method which can be called on this struct
// impl MigrationPlan {
//     pub fn differences(&self) -> String {
//         match self {
//             MigrationPlan::Create { .. } => "create".to_string(),
//             MigrationPlan::Update { .. } => "update".to_string(),
//             MigrationPlan::Delete { .. } => "delete".to_string(),
//         }
//     }
// }

impl SwimlaneMigrator {
    pub fn new(
        from: SwimlaneClient,
        to: SwimlaneClient,
        dry_run: bool,
    ) -> Result<Self, SwimlaneMigratorNewError> {
        if from.base_url == to.base_url {
            return Err(SwimlaneMigratorNewError::SourceAndDestinationAreIdentical);
        }
        Ok(SwimlaneMigrator { from, to, dry_run })
    }

    /// Transforms a group's properties to match that of the destination system.
    /// This will allow us to
    // async fn adapt_group(
    //     &self,
    //     group: &swimlane::Group,
    // ) -> Result<swimlane::Group, SwimlaneMigratorError> {
    //     let mut group = group.clone();

    //     // let mut users = vec![];
    //     // for user in group.users {
    //     //     let user = self.to.get_user_by_username(&user).await?;
    //     //     users.push(user.user_name);
    //     // }
    //     // group.users = users;

    //     // let mut groups = vec![];
    //     // for group in group.groups {
    //     //     let group = self.to.get_group_by_name(&group).await?;
    //     //     groups.push(group.name);
    //     // }
    //     // group.groups = groups;

    //     Ok(group)
    // }

    pub async fn migrate_apps(&self) {
        todo!()
    }
}
