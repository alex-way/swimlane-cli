use crate::equality::LooksLike;
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

impl SwimlaneMigrator {
    pub async fn get_groups_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<swimlane::groups::Group>>, SwimlaneMigratorError> {
        let source_groups = self.from.get_groups();
        let destination_groups = self.to.get_groups();

        let source_groups = source_groups.await?;
        let destination_groups = destination_groups.await?;

        let mut groups_to_migrate = vec![];

        for source_group in source_groups {
            if let Some(_destination_group) = destination_groups.iter().find(|destination_group| {
                destination_group.name.to_lowercase() == source_group.name.to_lowercase()
            }) {
                if source_group.looks_like(_destination_group) {
                    continue;
                }
                groups_to_migrate.push(MigrationPlan::Update {
                    source_resource: source_group,
                    destination_resource: _destination_group.clone(),
                });
            } else {
                groups_to_migrate.push(MigrationPlan::Create {
                    source_resource: source_group,
                });
            }
        }

        Ok(groups_to_migrate)
    }

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
}
