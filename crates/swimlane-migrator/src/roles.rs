use swimlane::roles::Role;

use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

impl LooksLike for Role {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut differences = vec![];

        if self.disabled != other.disabled {
            differences.push(Difference::UpdatingField {
                field: "disabled".to_string(),
                current_value: self.disabled.to_string(),
                new_value: other.disabled.to_string(),
            });
        }

        if self.description != other.description {
            differences.push(Difference::UpdatingField {
                field: "description".to_string(),
                current_value: match &self.description {
                    Some(description) => description.clone(),
                    None => "".to_string(),
                },
                new_value: match &other.description {
                    Some(description) => description.clone(),
                    None => "".to_string(),
                },
            });
        }

        differences
    }

    fn is_same_resource(&self, other: &Self) -> bool {
        self.name == other.name
    }

    // fn looks_like(&self, other: &Self) -> bool {
    //     // todo: compare group membership, role membership, permissions
    //     self.disabled == other.disabled && self.description == other.description
    // }
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
