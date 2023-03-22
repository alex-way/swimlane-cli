use crate::equality::LooksLike;
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

impl SwimlaneMigrator {
    pub async fn get_users_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<swimlane::users::User>>, SwimlaneMigratorError> {
        let source_users = self.from.get_users();
        let destination_users = self.to.get_users();

        let source_users = source_users.await?;
        let destination_users = destination_users.await?;

        let mut users_to_migrate = vec![];

        for source_user in source_users {
            if let Some(_destination_user) = destination_users.iter().find(|destination_user| {
                destination_user.user_name.to_lowercase() == source_user.user_name.to_lowercase()
            }) {
                if source_user.looks_like(_destination_user) {
                    continue;
                }
                users_to_migrate.push(MigrationPlan::Update {
                    source_resource: source_user,
                    destination_resource: _destination_user.clone(),
                });
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
}
