use std::collections::HashMap;

use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

use swimlane::users::{User, UserCreationRequest};

impl LooksLike for User {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut diffs = vec![];

        push_difference!(diffs, "display_name", &self.display_name, &other.display_name, optional: true);
        push_difference!(diffs, "disabled", &self.disabled, &other.disabled);
        // todo: Convert from ID to name somehow
        // push_difference!(diffs, "default_workspace_id", &self.default_workspace_id, &other.default_workspace_id, optional: true);
        // todo: Convert from ID to name somehow
        // push_difference!(diffs, "default_dashboard_id", &self.default_dashboard_id, &other.default_dashboard_id, optional: true);
        // todo: Reduce the migrationplan output for avatar as it's a base64encoded string.
        if self.avatar != other.avatar {
            diffs.push(Difference::UpdatingComplexField {
                field: "avatar".to_string(),
            });
        }
        push_difference!(diffs, "timezone_id", &self.timezone_id, &other.timezone_id);
        push_difference!(diffs, "email", &self.email, &other.email);
        push_difference!(diffs, "phone_number", &self.phone_number, &other.phone_number, optional: true);
        push_difference!(diffs, "first_name", &self.first_name, &other.first_name, optional: true);
        push_difference!(diffs, "middle_initial", &self.middle_initial, &other.middle_initial, optional: true);
        push_difference!(diffs, "last_name", &self.last_name, &other.last_name, optional: true);
        push_difference!(diffs, "roles", &self.roles, &other.roles, vec: true);
        push_difference!(diffs, "groups", &self.groups, &other.groups, vec: true);
        push_difference!(diffs, "primary_group", &self.primary_group, &other.primary_group, optional: true);

        diffs
    }

    fn is_same_resource(&self, other: &Self) -> bool {
        self.user_name == other.user_name
    }
}

impl SwimlaneMigrator {
    pub async fn get_users_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<User>>, SwimlaneMigratorError> {
        let source_users_future = self.from.get_users();
        let target_users_future = self.to.get_users();

        self.get_resources_to_migrate(source_users_future, target_users_future)
            .await
    }

    // todo: add argument to auto-create missing groups, roles, apps, dashboards
    // todo: Take an argument of Vec<MigrationPlan> and only migrate those resources?
    pub async fn migrate_users(&self) -> Result<(), SwimlaneMigratorError> {
        let users_to_migrate = self.get_users_to_migrate();
        let group_id_hashmap = self.get_group_id_hashmap();
        let role_id_hashmap = self.get_role_id_hashmap();

        let users_to_migrate = users_to_migrate.await?;
        let group_id_hashmap = group_id_hashmap.await?;
        let role_id_hashmap = role_id_hashmap.await?;

        // todo: handle pre-requisites/dependencies (groups, roles, apps, dashboards)

        // Delete users first, in order to ensure enough free licenses.
        for user in &users_to_migrate {
            if let MigrationPlan::Delete { target_resource } = user {
                self.to.delete_user(&target_resource.id).await?;
            }
        }

        // todo: parellelize
        for plan in &users_to_migrate {
            match plan {
                MigrationPlan::Create { source_resource } => {
                    let mut adapted_user = source_resource.clone();
                    self.adapt_user(&mut adapted_user, &group_id_hashmap, &role_id_hashmap);
                    let user_creation_request = UserCreationRequest::from(adapted_user);
                    self.to.create_user(&user_creation_request).await?;
                }
                MigrationPlan::Update {
                    source_resource,
                    target_resource,
                } => {
                    let mut adapted_user = source_resource.clone();
                    self.adapt_user(&mut adapted_user, &group_id_hashmap, &role_id_hashmap);
                    adapted_user.id = target_resource.id.clone();
                    self.to.update_user(&adapted_user).await?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn adapt_user(
        &self,
        user: &mut User,
        group_id_hashmap: &HashMap<String, String>,
        role_id_hashmap: &HashMap<String, String>,
    ) {
        // todo: handle default_workspace_id, default_dashboard_id
        for role in &mut user.roles {
            if let Some(new_id) = role_id_hashmap.get(&role.id) {
                role.id = new_id.clone();
            }
        }

        for group in &mut user.groups {
            if let Some(new_id) = group_id_hashmap.get(&group.id) {
                group.id = new_id.clone();
            }
        }
    }
}
