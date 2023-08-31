use std::collections::HashMap;

use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

use swimlane::groups::Group;

impl LooksLike for Group {
    /// Hello world
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut differences = vec![];

        // Add warning if the name is different
        push_difference!(differences, "name", &self.name, &other.name);
        push_difference!(differences, "disabled", &self.disabled, &other.disabled);
        push_difference!(differences, "description", &self.description, &other.description, optional: true);
        push_difference!(differences, "users", &self.users, &other.users, vec: true);
        push_difference!(differences, "roles", &self.roles, &other.roles, vec: true);
        push_difference!(differences, "groups", &self.groups, &other.groups, vec: true);

        differences
    }

    fn is_same_resource(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl SwimlaneMigrator {
    pub async fn get_groups_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<Group>>, SwimlaneMigratorError> {
        let source_groups_future = self.from.get_groups();
        let target_groups_future = self.to.get_groups();

        self.get_resources_to_migrate(source_groups_future, target_groups_future)
            .await
    }

    // Migrate group
    // Create a placeholder group, with the fields which aren't dependent on other resources. I.e. name, description, etc.
    // Then, create the nested groups, roles, and users
    // async fn migrate_group(&self, group: &Group) -> Result<(), SwimlaneMigratorError> {
    //     let mut group_to_create = group.clone();

    //     let mut nested_groups = vec![];
    //     for group in &mut group_to_create.groups {
    //         let group = self.migrate_group(group).await?;
    //         nested_groups.push(group);
    //     }
    //     group_to_create.groups = nested_groups;

    //     let mut roles = vec![];
    //     for role in &mut group_to_create.roles {
    //         let role = self.migrate_role(role).await?;
    //         roles.push(role);
    //     }
    //     group_to_create.roles = roles;

    //     let mut users = vec![];
    //     for user in &mut group_to_create.users {
    //         let user = self.migrate_user(user).await?;
    //         users.push(user);
    //     }
    //     group_to_create.users = users;

    //     self.to.create_group(&group_to_create).await?;

    //     Ok(())
    // }

    pub async fn migrate_groups(&self) -> Result<(), SwimlaneMigratorError> {
        let groups_to_migrate = self.get_groups_to_migrate();
        let group_id_hashmap = self.get_group_id_hashmap();
        let user_id_hashmap = self.get_user_id_hashmap();
        let role_id_hashmap = self.get_role_id_hashmap();

        let groups_to_migrate = groups_to_migrate.await?;
        let group_id_hashmap = group_id_hashmap.await?;
        let user_id_hashmap = user_id_hashmap.await?;
        let role_id_hashmap = role_id_hashmap.await?;

        // Perform all creates/deleted first as the groups may be referenced by other groups
        // todo: parellelize creation of groups
        for group in &groups_to_migrate {
            if let MigrationPlan::Create { source_resource } = group {
                let mut group_to_create = source_resource.clone();

                let mut nested_groups = vec![];
                for group in &mut group_to_create.groups {
                    if group_id_hashmap.contains_key(&group.id) {
                        group.id = group_id_hashmap[&group.id].clone();
                    } else {
                        return Err(SwimlaneMigratorError::MissingGroup {
                            group_name: group.name.clone(),
                        });
                    }
                    nested_groups.push(group.clone());
                }
                group_to_create.groups = nested_groups;

                let mut roles = vec![];
                for role in &mut group_to_create.roles {
                    if role_id_hashmap.contains_key(&role.id) {
                        role.id = role_id_hashmap[&role.id].clone();
                    } else {
                        return Err(SwimlaneMigratorError::MissingRole {
                            role_name: role.name.clone(),
                        });
                    }
                    roles.push(role.clone());
                }
                group_to_create.roles = roles;

                let mut users = vec![];
                for user in &mut group_to_create.users {
                    if user_id_hashmap.contains_key(&user.id) {
                        user.id = user_id_hashmap[&user.id].clone();
                    } else {
                        return Err(SwimlaneMigratorError::MissingUser {
                            user_name: user.name.clone(),
                        });
                    }
                    users.push(user.clone());
                }
                group_to_create.users = users;
                self.to.create_group(&group_to_create).await?;
            }
        }

        for group in &groups_to_migrate {
            match group {
                MigrationPlan::Create { source_resource } => {
                    // replace the id in each of the nested groups with the id from the target system
                    // let mut groups = vec![];
                    // replace the id in each of the nested roles with the id from the target system
                    // let mut roles = vec![];
                    // replace the id in each of the nested users with the id from the target system
                    // let mut users = vec![];
                    println!("  {} (create)", source_resource.name);
                }
                MigrationPlan::Update {
                    source_resource,
                    target_resource,
                } => {
                    println!(
                        "  {} (update) - {}",
                        source_resource.name, target_resource.name
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Adapts a group from a source system to a target system
    /// This is used to replace the ids of nested groups, roles, and users with the ids from the target system
    /// This is necessary because the ids of these resources are not guaranteed to be the same between systems
    ///
    /// Example:
    ///
    /// ```rust
    /// use swimlane_migrator::SwimlaneMigrator;
    /// use swimlane::SwimlaneClient;
    /// use swimlane::groups::Group;
    /// use std::collections::HashMap;
    ///
    /// let mut group = Group::default();
    ///
    /// group.id = "1234".to_string();
    ///
    /// let mut group_id_hashmap = HashMap::new();
    /// group_id_hashmap.insert("1234".to_string(), "9012".to_string());
    ///
    /// let user_id_hashmap = HashMap::new();
    ///
    /// let role_id_hashmap = HashMap::new();
    ///
    /// let source_swimlane = SwimlaneClient::new("https://source.swimlane.com".to_string(), "source_api_key".to_string());
    ///
    /// let target_swimlane = SwimlaneClient::new("https://target.swimlane.com".to_string(), "target_api_key".to_string());
    ///
    /// let migrator = SwimlaneMigrator::new(source_swimlane, target_swimlane, false).expect("Failed to create migrator");
    /// migrator.adapt_group(&mut group, &group_id_hashmap, &user_id_hashmap, &role_id_hashmap);
    ///
    /// assert_eq!(group.id, "9012");
    /// ```
    pub fn adapt_group(
        &self,
        group: &mut Group,
        group_id_hashmap: &HashMap<String, String>,
        user_id_hashmap: &HashMap<String, String>,
        role_id_hashmap: &HashMap<String, String>,
    ) {
        if let Some(new_id) = group_id_hashmap.get(&group.id) {
            group.id = new_id.clone();
        }

        for user in &mut group.users {
            if let Some(new_id) = user_id_hashmap.get(&user.id) {
                user.id = new_id.clone();
            }
        }

        for role in &mut group.roles {
            if let Some(new_id) = role_id_hashmap.get(&role.id) {
                role.id = new_id.clone();
            }
        }

        for child_group in &mut group.groups {
            if let Some(new_id) = group_id_hashmap.get(&child_group.id) {
                child_group.id = new_id.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use swimlane::BaseEntity;

    use super::*;

    #[test]
    fn test_name_inequality_triggers_difference() {
        let mut source_group = Group::default();
        let mut target_group = Group::default();
        source_group.name = "Group 1".to_string();
        target_group.name = "Group 2".to_string();

        let differences = source_group.differences(&target_group);
        assert_eq!(differences.len(), 1);
        assert_eq!(
            differences[0],
            Difference::UpdatingField {
                field: "name".to_string(),
                current_value: "Group 1".to_string(),
                new_value: "Group 2".to_string(),
            }
        );
    }

    #[test]
    fn test_disabled_inequality_triggers_difference() {
        let mut source_group = Group::default();
        let mut target_group = Group::default();
        source_group.disabled = true;
        target_group.disabled = false;

        let differences = source_group.differences(&target_group);
        assert_eq!(differences.len(), 1);
        assert_eq!(
            differences[0],
            Difference::UpdatingField {
                field: "disabled".to_string(),
                current_value: true.to_string(),
                new_value: false.to_string(),
            }
        );
    }

    #[test]
    fn test_description_inequality_triggers_difference() {
        let mut source_group = Group::default();
        let mut target_group = Group::default();
        source_group.description = Some("Description 1".to_string());
        target_group.description = Some("Description 2".to_string());

        let differences = source_group.differences(&target_group);
        assert_eq!(differences.len(), 1);
        assert_eq!(
            differences[0],
            Difference::UpdatingField {
                field: "description".to_string(),
                current_value: "Description 1".to_string(),
                new_value: "Description 2".to_string(),
            }
        );
    }

    #[test]
    fn test_users_inequality_triggers_difference() {
        let mut source_group = Group::default();
        let target_group = Group::default();
        source_group.users = vec![BaseEntity {
            id: "1234".to_string(),
            name: "User 1".to_string(),
            disabled: false,
        }];

        let differences = source_group.differences(&target_group);
        assert_eq!(differences.len(), 1);
        assert_eq!(
            differences[0],
            Difference::AddingItem {
                field: "users".to_string(),
                item: "User 1".to_string()
            }
        )
    }

    #[test]
    fn test_roles_inequality_triggers_difference() {
        let mut source_group = Group::default();
        let target_group = Group::default();
        source_group.roles = vec![BaseEntity {
            id: "1234".to_string(),
            name: "Role 1".to_string(),
            disabled: false,
        }];

        let differences = source_group.differences(&target_group);
        assert_eq!(differences.len(), 1);
        assert_eq!(
            differences[0],
            Difference::AddingItem {
                field: "roles".to_string(),
                item: "Role 1".to_string()
            }
        )
    }
}
