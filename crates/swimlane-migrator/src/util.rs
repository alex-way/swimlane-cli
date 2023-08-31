use swimlane::error::SwimlaneClientError;

use crate::{equality::LooksLike, MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

use std::{collections::HashMap, future::Future};

impl SwimlaneMigrator {
    /// Returns a hashmap of id to id for all groups present in both the source and target systems
    pub async fn get_group_id_hashmap(
        &self,
    ) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_groups = self.from.get_groups();
        let target_groups = self.to.get_groups();

        let source_groups = source_groups.await?;
        let target_groups = target_groups.await?;

        let mut group_id_hashmap = HashMap::new();

        for source_group in source_groups {
            if let Some(target_group) = target_groups.iter().find(|target_group| {
                target_group.name.to_lowercase() == source_group.name.to_lowercase()
            }) {
                group_id_hashmap.insert(source_group.id.clone(), target_group.id.clone());
            }
        }

        Ok(group_id_hashmap)
    }

    /// Returns a hashmap of id to id for all users present in both the source and target systems
    pub async fn get_user_id_hashmap(
        &self,
    ) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_users = self.from.get_users();
        let target_users = self.to.get_users();

        let source_users = source_users.await?;
        let target_users = target_users.await?;

        let mut hashmap = HashMap::new();

        for source_user in source_users {
            if let Some(target_user) = target_users.iter().find(|target_user| {
                target_user.user_name.to_lowercase() == source_user.user_name.to_lowercase()
            }) {
                hashmap.insert(source_user.id.clone(), target_user.id.clone());
            }
        }

        Ok(hashmap)
    }

    /// Returns a hashmap of id to id for all roles present in both the source and target systems
    pub async fn get_role_id_hashmap(
        &self,
    ) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_roles = self.from.get_roles();
        let target_roles = self.to.get_roles();

        let source_roles = source_roles.await?;
        let target_roles = target_roles.await?;

        let mut hashmap = HashMap::new();

        for source_role in source_roles {
            if let Some(target_role) = target_roles.iter().find(|target_role| {
                target_role.name.to_lowercase() == source_role.name.to_lowercase()
            }) {
                hashmap.insert(source_role.id.clone(), target_role.id.clone());
            }
        }

        Ok(hashmap)
    }

    pub async fn get_task_hashmap(&self) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_tasks = self.from.get_tasks_light();
        let target_tasks = self.to.get_tasks_light();

        let source_tasks = source_tasks.await?;
        let target_tasks = target_tasks.await?;

        let mut hashmap = HashMap::new();

        for source_task in source_tasks {
            if let Some(target_task) = target_tasks.iter().find(|target_task| {
                target_task.name.to_lowercase() == source_task.name.to_lowercase()
            }) {
                hashmap.insert(source_task.id.clone(), target_task.id.clone());
            }
        }

        Ok(hashmap)
    }

    pub async fn get_application_hashmap(
        &self,
    ) -> Result<HashMap<String, String>, SwimlaneMigratorError> {
        let source_applications = self.from.get_applications_light();
        let target_applications = self.to.get_applications_light();

        let source_applications = source_applications.await?;
        let target_applications = target_applications.await?;

        let mut hashmap = HashMap::new();

        for source_application in source_applications {
            if let Some(target_application) =
                target_applications.iter().find(|target_application| {
                    target_application.name.to_lowercase() == source_application.name.to_lowercase()
                })
            {
                hashmap.insert(source_application.id.clone(), target_application.id.clone());
            }
        }

        Ok(hashmap)
    }

    pub async fn get_resources_to_migrate<T: LooksLike + Clone, FutSrc, FutDest>(
        &self,
        source_resource_getter: FutSrc,
        target_resource_getter: FutDest,
    ) -> Result<Vec<MigrationPlan<T>>, SwimlaneMigratorError>
    where
        FutSrc: Future<Output = Result<Vec<T>, SwimlaneClientError>>,
        FutDest: Future<Output = Result<Vec<T>, SwimlaneClientError>>,
    {
        let source_resources = source_resource_getter.await?;
        let target_resources = target_resource_getter.await?;

        self._get_resources_to_migrate(source_resources, target_resources)
    }

    pub fn _get_resources_to_migrate<T: LooksLike + Clone>(
        &self,
        source_resources: Vec<T>,
        target_resources: Vec<T>,
    ) -> Result<Vec<MigrationPlan<T>>, SwimlaneMigratorError> {
        let mut resources_to_migrate = vec![];

        for source_resource in source_resources.clone() {
            if let Some(target_resource) = target_resources
                .iter()
                .find(|target_resource| target_resource.is_same_resource(&source_resource))
            {
                if source_resource.looks_like(target_resource) {
                    continue;
                }
                resources_to_migrate.push(MigrationPlan::Update {
                    source_resource,
                    target_resource: target_resource.clone(),
                });
            } else {
                resources_to_migrate.push(MigrationPlan::Create { source_resource });
            }
        }

        for target_resource in target_resources {
            if !source_resources
                .iter()
                .any(|source_resource| source_resource.is_same_resource(&target_resource))
            {
                resources_to_migrate.push(MigrationPlan::Delete { target_resource });
            }
        }

        Ok(resources_to_migrate)
    }
}
