pub mod equality;

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
    // todo: turn the differences into a method which can be called on this struct
    // pub differences: Vec<String>,
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
        Ok(SwimlaneMigrator { from, to, dry_run })
    }

    // todo: return the source user, destination user, a list of differences and an action (create, update, delete)
    pub async fn get_users_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<swimlane::User>>, SwimlaneMigratorError> {
        let source_users = self.from.get_users();
        let destination_users = self.to.get_users();

        let source_users = source_users.await?;
        let destination_users = destination_users.await?;

        let mut users_to_migrate = vec![];

        for source_user in source_users {
            if let Some(_destination_user) = destination_users.iter().find(|destination_user| {
                destination_user.user_name.to_lowercase() == source_user.user_name.to_lowercase()
            }) {
                if !source_user.looks_like(_destination_user) {
                    users_to_migrate.push(MigrationPlan::Update {
                        source_resource: source_user,
                        destination_resource: _destination_user.clone(),
                    });
                }
            } else {
                users_to_migrate.push(MigrationPlan::Create {
                    source_resource: source_user,
                });
            }
        }

        Ok(users_to_migrate)
    }

    // todo: add argument to auto-create missing groups, roles, apps, dashboards
    pub async fn migrate_users(&self) -> Result<(), SwimlaneMigratorError> {
        let users_to_migrate = self.get_users_to_migrate().await?;

        // todo: handle pre-requisites/dependencies (groups, roles, apps, dashboards)

        println!("Users to migrate:");
        for user in &users_to_migrate {
            match user {
                MigrationPlan::Create { source_resource } => {
                    println!("  {} (create)", source_resource.user_name);
                }
                MigrationPlan::Update {
                    source_resource,
                    destination_resource,
                } => {
                    println!(
                        "  {} (update) - {}",
                        source_resource.user_name, destination_resource.user_name
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }

    // todo: return the source user, destination user, a list of differences and an action (create, update, delete)
    pub async fn get_groups_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<swimlane::Group>>, SwimlaneMigratorError> {
        let source_groups = self.from.get_groups();
        let destination_groups = self.to.get_groups();

        let source_groups = source_groups.await?;
        let destination_groups = destination_groups.await?;

        let mut groups_to_migrate = vec![];

        for source_group in source_groups {
            if let Some(_destination_group) = destination_groups.iter().find(|destination_group| {
                destination_group.name.to_lowercase() == source_group.name.to_lowercase()
            }) {
                if !source_group.looks_like(_destination_group) {
                    groups_to_migrate.push(MigrationPlan::Update {
                        source_resource: source_group,
                        destination_resource: _destination_group.clone(),
                    });
                }
            } else {
                groups_to_migrate.push(MigrationPlan::Create {
                    source_resource: source_group,
                });
            }
        }

        Ok(groups_to_migrate)
    }

    pub async fn migrate_groups(&self) -> Result<(), SwimlaneMigratorError> {
        let groups_to_migrate = self.get_groups_to_migrate().await?;

        println!("Groups to migrate:");
        for group in groups_to_migrate {
            match group {
                MigrationPlan::Create { source_resource } => {
                    println!("  {} (create)", source_resource.name);
                }
                MigrationPlan::Update {
                    source_resource,
                    destination_resource,
                } => {
                    println!(
                        "  {} (update) - {}",
                        source_resource.name, destination_resource.name
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub async fn migrate_roles(&self) {
        todo!()
    }

    pub async fn migrate_apps(&self) {
        todo!()
    }
}
