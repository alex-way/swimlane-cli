use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};
use swimlane::users::User;

impl LooksLike for User {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut differences = vec![];

        if self.display_name != other.display_name {
            differences.push(Difference::UpdatingField {
                field: "display_name".to_string(),
                current_value: match &self.display_name {
                    Some(display_name) => display_name.clone(),
                    None => "".to_string(),
                },
                new_value: match &other.display_name {
                    Some(display_name) => display_name.clone(),
                    None => "".to_string(),
                },
            });
        }

        if self.disabled != other.disabled {
            differences.push(Difference::UpdatingField {
                field: "disabled".to_string(),
                current_value: self.disabled.to_string(),
                new_value: other.disabled.to_string(),
            });
        }

        if self.email != other.email {
            differences.push(Difference::UpdatingField {
                field: "email".to_string(),
                current_value: self.email.clone(),
                new_value: other.email.clone(),
            });
        }

        if self.first_name != other.first_name {
            differences.push(Difference::UpdatingField {
                field: "first_name".to_string(),
                current_value: match &self.first_name {
                    Some(first_name) => first_name.clone(),
                    None => "".to_string(),
                },
                new_value: match &other.first_name {
                    Some(first_name) => first_name.clone(),
                    None => "".to_string(),
                },
            });
        }

        if self.last_name != other.last_name {
            differences.push(Difference::UpdatingField {
                field: "last_name".to_string(),
                current_value: match &self.last_name {
                    Some(last_name) => last_name.clone(),
                    None => "".to_string(),
                },
                new_value: match &other.last_name {
                    Some(last_name) => last_name.clone(),
                    None => "".to_string(),
                },
            });
        }

        differences
    }

    fn is_same_resource(&self, other: &Self) -> bool {
        self.user_name == other.user_name
    }

    // fn looks_like(&self, other: &Self) -> bool {
    //     // todo: compare group membership, role membership, primary group, phone, time zone, default dashboard, profile image, middle initial
    //     self.display_name == other.display_name
    //         && self.disabled == other.disabled
    //         && self.email == other.email
    //         && self.first_name == other.first_name
    //         && self.last_name == other.last_name
    // }
}

impl SwimlaneMigrator {
    pub async fn get_users_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<User>>, SwimlaneMigratorError> {
        let source_users_future = self.from.get_users();
        let destination_users_future = self.to.get_users();

        self.get_resources_to_migrate(source_users_future, destination_users_future)
            .await
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
