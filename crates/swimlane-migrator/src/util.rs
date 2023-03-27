use swimlane::error::SwimlaneClientError;

use crate::{equality::LooksLike, MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

use std::{collections::HashMap, future::Future};

impl SwimlaneMigrator {
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

        let mut hashmap = HashMap::new();

        for source_user in source_users {
            if let Some(destination_user) = destination_users.iter().find(|destination_user| {
                destination_user.user_name.to_lowercase() == source_user.user_name.to_lowercase()
            }) {
                hashmap.insert(source_user.id.clone(), destination_user.id.clone());
            }
        }

        Ok(hashmap)
    }

    /// Returns a hashmap of id to id for all roles present in both the source and destination systems
    pub async fn get_role_id_hashmap(
        &self,
    ) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_roles = self.from.get_roles();
        let destination_roles = self.to.get_roles();

        let source_roles = source_roles.await?;
        let destination_roles = destination_roles.await?;

        let mut hashmap = HashMap::new();

        for source_role in source_roles {
            if let Some(destination_role) = destination_roles.iter().find(|destination_role| {
                destination_role.name.to_lowercase() == source_role.name.to_lowercase()
            }) {
                hashmap.insert(source_role.id.clone(), destination_role.id.clone());
            }
        }

        Ok(hashmap)
    }

    pub async fn get_task_hashmap(&self) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_tasks = self.from.get_tasks_light();
        let destination_tasks = self.to.get_tasks_light();

        let source_tasks = source_tasks.await?;
        let destination_tasks = destination_tasks.await?;

        let mut hashmap = HashMap::new();

        for source_task in source_tasks {
            if let Some(destination_task) = destination_tasks.iter().find(|destination_task| {
                destination_task.name.to_lowercase() == source_task.name.to_lowercase()
            }) {
                hashmap.insert(source_task.id.clone(), destination_task.id.clone());
            }
        }

        Ok(hashmap)
    }

    pub async fn get_application_hashmap(
        &self,
    ) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_applications = self.from.get_applications_light();
        let destination_applications = self.to.get_applications_light();

        let source_applications = source_applications.await?;
        let destination_applications = destination_applications.await?;

        let mut hashmap = HashMap::new();

        for source_application in source_applications {
            if let Some(destination_application) =
                destination_applications
                    .iter()
                    .find(|destination_application| {
                        destination_application.name.to_lowercase()
                            == source_application.name.to_lowercase()
                    })
            {
                hashmap.insert(
                    source_application.id.clone(),
                    destination_application.id.clone(),
                );
            }
        }

        Ok(hashmap)
    }

    pub async fn get_resources_to_migrate<T: LooksLike + Clone, FutSrc, FutDest>(
        &self,
        source_resource_getter: FutSrc,
        destination_resource_getter: FutDest,
    ) -> Result<Vec<MigrationPlan<T>>, SwimlaneMigratorError>
    where
        FutSrc: Future<Output = Result<Vec<T>, SwimlaneClientError>>,
        FutDest: Future<Output = Result<Vec<T>, SwimlaneClientError>>,
    {
        let source_resources = source_resource_getter.await?;
        let destination_resources = destination_resource_getter.await?;

        let mut resources_to_migrate = vec![];

        for source_resource in source_resources.clone() {
            if let Some(destination_resource) =
                destination_resources.iter().find(|destination_resource| {
                    destination_resource.is_same_resource(&source_resource)
                })
            {
                if source_resource.looks_like(destination_resource) {
                    continue;
                }
                resources_to_migrate.push(MigrationPlan::Update {
                    source_resource,
                    destination_resource: destination_resource.clone(),
                });
            } else {
                resources_to_migrate.push(MigrationPlan::Create { source_resource });
            }
        }

        for destination_resource in destination_resources {
            if !source_resources
                .iter()
                .any(|source_resource| source_resource.is_same_resource(&destination_resource))
            {
                resources_to_migrate.push(MigrationPlan::Delete {
                    destination_resource,
                });
            }
        }

        Ok(resources_to_migrate)
    }
}
