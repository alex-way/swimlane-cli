use std::collections::hash_map;

use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};
use swimlane::workspaces::Workspace;

impl LooksLike for Workspace {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut diffs = vec![];

        push_difference!(diffs, "name", &self.name, &other.name);
        push_difference!(diffs, "description", &self.description, &other.description, optional: true);
        push_difference!(diffs, "disabled", &self.disabled, &other.disabled);
        /// TODO : We need to ensure that we conver the list of ids to list of names before performing the comparison.
        // push_difference!(diffs, "dashboards", &self.dashboards, &other.dashboards);
        // push_difference!(diffs, "applications", &self.applications, &other.applications);
        diffs
    }

    fn is_same_resource(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl SwimlaneMigrator {
    pub async fn get_workspaces_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<Workspace>>, SwimlaneMigratorError> {
        println!("Getting workspaces to migrate");
        // let source_workspace_future = self.from.get_roles();
        // let destination_workspace_future = self.to.get_roles();

        // self.get_resources_to_migrate(source_roles_future, destination_roles_future)
        //     .await
        todo!()
    }

    pub async fn migrate_workspaces(&self) -> Result<(), SwimlaneMigratorError> {
        todo!()
    }
}
