mod adapt;
#[macro_use]
pub mod equality;
pub mod apps;
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
    from_normaliser: adapt::SwimlaneResourceNormaliser,
    to_normaliser: adapt::SwimlaneResourceNormaliser,
}

impl SwimlaneMigrator {
    pub fn new(
        from: SwimlaneClient,
        to: SwimlaneClient,
        dry_run: bool,
    ) -> Result<Self, SwimlaneMigratorNewError> {
        if from.base_url == to.base_url {
            return Err(SwimlaneMigratorNewError::SourceAndDestinationAreIdentical);
        }
        let from_normaliser = adapt::SwimlaneResourceNormaliser::new(from.clone());
        let to_normaliser = adapt::SwimlaneResourceNormaliser::new(to.clone());
        Ok(SwimlaneMigrator {
            from,
            to,
            dry_run,
            from_normaliser,
            to_normaliser,
        })
    }
}

#[derive(Error, Debug)]
pub enum SwimlaneMigratorNewError {
    #[error(
        "The source and target Swimlane servers are the same. Please specify different servers."
    )]
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
        target_resource: T,
    },
    Delete {
        target_resource: T,
    },
}
