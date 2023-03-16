pub mod equality;

use std::collections::HashMap;

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

    // logger.info("Migrating Groups...")
    // source_groups = self.source.groups.list()
    // destination_groups = self.destination.groups.list()
    // destination_group_names = [g.name for g in destination_groups]

    // # First create any missing groups
    // for s_group in source_groups:
    //     if s_group.name in destination_group_names:
    //         continue
    //     logger.info(f"Creating Group: {s_group.name}")
    //     source_definition = s_group._raw
    //     definition = {
    //         "name": s_group.name,
    //         "description": source_definition.get("description", ""),
    //     }
    //     response = self.destination.request("post", "groups", json=definition)
    //     response.raise_for_status()

    // # Next update all groups with the required sub-groups
    // # Build a dictionary for each group
    // group_mapping = {}
    // for s_group in source_groups:
    //     group_mapping[s_group.name] = [g["name"] for g in s_group._raw["groups"]]

    // # Refresh the destination groups
    // destination_groups = self.destination.groups.list()
    // # Update all groups with the subgroups
    // for d_group in destination_groups:
    //     definition = d_group._raw
    //     definition["groups"] = [{"name": g} for g in group_mapping[d_group.name]]
    //     logger.info(f"Updating group {d_group.name}")
    //     self.destination.request("put", f"groups/{d_group.id}", json=definition)

    /// Returns a hashmap of id to id for all groups present in both the source and destination systems
    pub async fn get_group_id_hashmap(
        &self,
    ) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_groups = self.from.get_groups();
        let destination_groups = self.to.get_groups();

        let source_groups = source_groups.await?;
        let destination_groups = destination_groups.await?;

        let mut group_id_hashmap = HashMap::new();

        for source_group in source_groups {
            if let Some(destination_group) = destination_groups.iter().find(|destination_group| {
                destination_group.name.to_lowercase() == source_group.name.to_lowercase()
            }) {
                group_id_hashmap.insert(source_group.id.clone(), destination_group.id.clone());
            }
        }

        Ok(group_id_hashmap)
    }

    /// Returns a hashmap of id to id for all users present in both the source and destination systems
    pub async fn get_user_id_hashmap(
        &self,
    ) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_users = self.from.get_users();
        let destination_users = self.to.get_users();

        let source_users = source_users.await?;
        let destination_users = destination_users.await?;

        let mut user_id_hashmap = HashMap::new();

        for source_user in source_users {
            if let Some(destination_user) = destination_users.iter().find(|destination_user| {
                destination_user.user_name.to_lowercase() == source_user.user_name.to_lowercase()
            }) {
                user_id_hashmap.insert(source_user.id.clone(), destination_user.id.clone());
            }
        }

        Ok(user_id_hashmap)
    }

    /// Returns a hashmap of id to id for all roles present in both the source and destination systems
    pub async fn get_role_id_hashmap(
        &self,
    ) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_roles = self.from.get_roles();
        let destination_roles = self.to.get_roles();

        let source_roles = source_roles.await?;
        let destination_roles = destination_roles.await?;

        let mut role_id_hashmap = HashMap::new();

        for source_role in source_roles {
            if let Some(destination_role) = destination_roles.iter().find(|destination_role| {
                destination_role.name.to_lowercase() == source_role.name.to_lowercase()
            }) {
                role_id_hashmap.insert(source_role.id.clone(), destination_role.id.clone());
            }
        }

        Ok(role_id_hashmap)
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
                // todo: parellelize creation of groups
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

    pub async fn migrate_roles(&self) {
        todo!()
    }

    pub async fn migrate_apps(&self) {
        todo!()
    }
}
