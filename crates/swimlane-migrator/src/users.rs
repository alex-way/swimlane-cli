use std::collections::HashMap;

use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

use swimlane::users::{User, UserCreationRequest};

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

        if self.middle_initial != other.middle_initial {
            differences.push(Difference::UpdatingField {
                field: "middle_initial".to_string(),
                current_value: match &self.middle_initial {
                    Some(middle_initial) => middle_initial.clone(),
                    None => "".to_string(),
                },
                new_value: match &other.middle_initial {
                    Some(middle_initial) => middle_initial.clone(),
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

        differences.extend(self.roles.iter().filter_map(|role| {
            let other_role_exists = other
                .roles
                .iter()
                .find(|other_role| role.looks_like(other_role));
            match other_role_exists {
                Some(_) => None,
                None => Some(Difference::AddingItem {
                    field: "roles".to_string(),
                    item: role.name.clone(),
                }),
            }
        }));

        differences.extend(other.roles.iter().filter_map(|role| {
            let role_exists = self
                .roles
                .iter()
                .find(|other_role| role.looks_like(other_role));
            match role_exists {
                Some(_) => None,
                None => Some(Difference::RemovingItem {
                    field: "roles".to_string(),
                    item: role.name.clone(),
                }),
            }
        }));

        differences.extend(self.groups.iter().filter_map(|group| {
            let other_group_exists = other
                .groups
                .iter()
                .find(|other_group| group.looks_like(other_group));
            match other_group_exists {
                Some(_) => None,
                None => Some(Difference::AddingItem {
                    field: "groups".to_string(),
                    item: group.name.clone(),
                }),
            }
        }));

        differences.extend(other.groups.iter().filter_map(|group| {
            let group_exists = self
                .groups
                .iter()
                .find(|other_group| group.looks_like(other_group));
            match group_exists {
                Some(_) => None,
                None => Some(Difference::RemovingItem {
                    field: "groups".to_string(),
                    item: group.name.clone(),
                }),
            }
        }));

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
    // todo: Take an argument of Vec<MigrationPlan> and only migrate those resources?
    pub async fn migrate_users(&self) -> Result<(), SwimlaneMigratorError> {
        let users_to_migrate = self.get_users_to_migrate();
        let group_id_hashmap = self.get_group_id_hashmap();
        let role_id_hashmap = self.get_role_id_hashmap();

        let users_to_migrate = users_to_migrate.await?;
        let group_id_hashmap = group_id_hashmap.await?;
        let role_id_hashmap = role_id_hashmap.await?;

        // todo: handle pre-requisites/dependencies (groups, roles, apps, dashboards)

        // Delete users first, in order to ensure enough free licenses.
        for user in &users_to_migrate {
            if let MigrationPlan::Delete {
                destination_resource,
            } = user
            {
                self.to.delete_user(&destination_resource.id).await?;
            }
        }

        // todo: parellelize
        for plan in &users_to_migrate {
            match plan {
                MigrationPlan::Create { source_resource } => {
                    let mut adapted_user = source_resource.clone();
                    self.adapt_user(&mut adapted_user, &group_id_hashmap, &role_id_hashmap);
                    let user_creation_request = UserCreationRequest::from(adapted_user);
                    self.to.create_user(&user_creation_request).await?;
                }
                MigrationPlan::Update {
                    source_resource,
                    destination_resource,
                } => {
                    let mut adapted_user = source_resource.clone();
                    self.adapt_user(&mut adapted_user, &group_id_hashmap, &role_id_hashmap);
                    adapted_user.id = destination_resource.id.clone();
                    self.to.update_user(&adapted_user).await?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn adapt_user(
        &self,
        user: &mut User,
        group_id_hashmap: &HashMap<String, String>,
        role_id_hashmap: &HashMap<String, String>,
    ) {
        for role in &mut user.roles {
            if let Some(new_id) = role_id_hashmap.get(&role.id) {
                role.id = new_id.clone();
            }
        }

        for group in &mut user.groups {
            if let Some(new_id) = group_id_hashmap.get(&group.id) {
                group.id = new_id.clone();
            }
        }
    }
}
