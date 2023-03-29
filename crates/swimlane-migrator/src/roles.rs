use swimlane::roles::Role;

use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

impl LooksLike for Role {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut differences = vec![];

        // Add warning if the name is different
        push_difference!(differences, "name", &self.name, &other.name);
        push_difference!(differences, "disabled", &self.disabled, &other.disabled);
        push_difference!(differences, "description", &self.description, &other.description, optional: true);
        push_difference!(differences, "users", &self.users, &other.users, vec: true);

        differences
    }

    fn is_same_resource(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl SwimlaneMigrator {
    pub async fn get_roles_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<Role>>, SwimlaneMigratorError> {
        let source_roles_future = self.from.get_roles();
        let destination_roles_future = self.to.get_roles();

        self.get_resources_to_migrate(source_roles_future, destination_roles_future)
            .await
    }

    pub async fn migrate_roles(&self) -> Result<(), SwimlaneMigratorError> {
        todo!()
    }
}
