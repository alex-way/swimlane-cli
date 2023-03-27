use std::collections::HashMap;

use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

use swimlane::groups::Group;

impl LooksLike for Group {
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
        let destination_groups_future = self.to.get_groups();

        self.get_resources_to_migrate(source_groups_future, destination_groups_future)
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
                    // replace the id in each of the nested groups with the id from the destination system
                    // let mut groups = vec![];
                    // replace the id in each of the nested roles with the id from the destination system
                    // let mut roles = vec![];
                    // replace the id in each of the nested users with the id from the destination system
                    // let mut users = vec![];
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

    /// Adapts a group from a source system to a destination system
    /// This is used to replace the ids of nested groups, roles, and users with the ids from the destination system
    /// This is necessary because the ids of these resources are not guaranteed to be the same between systems
    ///
    /// Example:
    ///
    /// ```rust
    /// use swimlane_migrator::SwimlaneMigrator;
    /// use swimlane::groups::Group;
    /// use std::collections::HashMap;
    ///
    /// let mut group = Group {
    ///    id: "1234".to_string(),
    ///   name: "Group 1".to_string(),
    ///  groups: vec![Group {
    ///    id: "5678".to_string(),
    ///   name: "Group 2".to_string(),
    /// groups: vec![],
    /// roles: vec![],
    /// users: vec![],
    /// }],
    /// roles: vec![],
    /// users: vec![],
    /// };
    ///
    /// let group_id_hashmap = HashMap::new();
    /// group_id_hashmap.insert("5678".to_string(), "9012".to_string());
    ///
    /// let user_id_hashmap = HashMap::new();
    ///
    /// let role_id_hashmap = HashMap::new();
    ///
    /// let mut migrator = SwimlaneMigrator::new();
    /// migrator.adapt_group(&mut group, &group_id_hashmap, &user_id_hashmap, &role_id_hashmap).await;
    ///
    /// assert_eq!(group.id, "1234");
    /// assert_eq!(group.groups[0].id, "9012");
    /// ```
    pub async fn adapt_group(
        &self,
        group: &mut Group,
        group_id_hashmap: &HashMap<String, String>,
        user_id_hashmap: &HashMap<String, String>,
        role_id_hashmap: &HashMap<String, String>,
    ) {
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
