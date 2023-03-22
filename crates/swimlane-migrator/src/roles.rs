use crate::equality::LooksLike;
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

impl SwimlaneMigrator {
    pub async fn get_roles_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<swimlane::roles::Role>>, SwimlaneMigratorError> {
        let source_roles = self.from.get_roles();
        let destination_roles = self.to.get_roles();

        let source_roles = source_roles.await?;
        let destination_roles = destination_roles.await?;

        let mut roles_to_migrate = vec![];

        for source_role in source_roles {
            if let Some(_destination_role) = destination_roles.iter().find(|destination_role| {
                destination_role.name.to_lowercase() == source_role.name.to_lowercase()
            }) {
                if source_role.looks_like(_destination_role) {
                    continue;
                }
                roles_to_migrate.push(MigrationPlan::Update {
                    source_resource: source_role,
                    destination_resource: _destination_role.clone(),
                });
            } else {
                roles_to_migrate.push(MigrationPlan::Create {
                    source_resource: source_role,
                });
            }
        }

        Ok(roles_to_migrate)
    }

    pub async fn migrate_roles(&self) {
        todo!()
    }
}
