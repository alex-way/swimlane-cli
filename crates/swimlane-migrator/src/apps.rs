use swimlane::apps::Application;

use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

impl LooksLike for Application {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut differences = vec![];

        // Add warning if the name is different
        push_difference!(differences, "name", &self.name, &other.name);
        push_difference!(differences, "disabled", &self.disabled, &other.disabled);
        push_difference!(differences, "description", &self.description, &other.description, optional: true);
        push_difference!(
            differences,
            "time_tracking_enabled",
            &self.time_tracking_enabled,
            &other.time_tracking_enabled
        );

        differences
    }

    fn is_same_resource(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl SwimlaneMigrator {
    pub async fn get_apps_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<Application>>, SwimlaneMigratorError> {
        let source_apps_future = self.from.get_applications();
        let destination_apps_future = self.to.get_applications();

        self.get_resources_to_migrate(source_apps_future, destination_apps_future)
            .await
    }

    pub async fn migrate_apps(&self) -> Result<(), SwimlaneMigratorError> {
        todo!()
    }
}
