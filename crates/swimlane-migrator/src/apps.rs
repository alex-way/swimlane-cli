use swimlane::apps::fields::Field;
use swimlane::apps::Application;

use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

impl LooksLike for Field {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let differences = vec![];

        if !self.is_same_resource(other) {
            return differences;
        }

        // todo: perform a mucn deeper comparison of fields
        // match (self, other) {
        //     (Field::SingleLineText(this_field), Field::SingleLineText(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::MultiLineText(this_field), Field::MultiLineText(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Email(this_field), Field::Email(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Telephone(this_field), Field::Telephone(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Url(this_field), Field::Url(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::IpAddress(this_field), Field::IpAddress(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::RichText(this_field), Field::RichText(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Json(this_field), Field::Json(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::TextList(this_field), Field::TextList(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Numeric(this_field), Field::Numeric(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::NumericList(this_field), Field::NumericList(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::DateTime(this_field), Field::DateTime(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Date(this_field), Field::Date(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Time(this_field), Field::Time(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::TimeSpan(this_field), Field::TimeSpan(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::FirstCreated(this_field), Field::FirstCreated(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::LastUpdated(this_field), Field::LastUpdated(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::SingleSelect(this_field), Field::SingleSelect(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::MultiSelect(this_field), Field::MultiSelect(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::RadioButtons(this_field), Field::RadioButtons(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Checkboxes(this_field), Field::Checkboxes(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::SingleUserGroup(this_field), Field::SingleUserGroup(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::MultiUserGroup(this_field), Field::MultiUserGroup(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::CreatedBy(this_field), Field::CreatedBy(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::LastUpdatedBy(this_field), Field::LastUpdatedBy(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Correlation(this_field), Field::Correlation(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::SingleReference(this_field), Field::SingleReference(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::MultiReference(this_field), Field::MultiReference(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::GridReference(this_field), Field::GridReference(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::TrackingId(this_field), Field::TrackingId(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Attachment(this_field), Field::Attachment(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::Comments(this_field), Field::Comments(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     (Field::History(this_field), Field::History(other_field)) => {
        //         this_field.differences(other_field)
        //     }
        //     _ => false,
        // }

        differences
    }

    fn is_same_resource(&self, other: &Self) -> bool {
        match (self, other) {
            (Field::SingleLineText(this_field), Field::SingleLineText(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::MultiLineText(this_field), Field::MultiLineText(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Email(this_field), Field::Email(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Telephone(this_field), Field::Telephone(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Url(this_field), Field::Url(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::IpAddress(this_field), Field::IpAddress(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::RichText(this_field), Field::RichText(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Json(this_field), Field::Json(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::TextList(this_field), Field::TextList(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Numeric(this_field), Field::Numeric(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::NumericList(this_field), Field::NumericList(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::DateTime(this_field), Field::DateTime(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Date(this_field), Field::Date(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Time(this_field), Field::Time(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::TimeSpan(this_field), Field::TimeSpan(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::FirstCreated(this_field), Field::FirstCreated(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::LastUpdated(this_field), Field::LastUpdated(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::SingleSelect(this_field), Field::SingleSelect(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::MultiSelect(this_field), Field::MultiSelect(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::RadioButtons(this_field), Field::RadioButtons(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Checkboxes(this_field), Field::Checkboxes(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::SingleUserGroup(this_field), Field::SingleUserGroup(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::MultiUserGroup(this_field), Field::MultiUserGroup(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::CreatedBy(this_field), Field::CreatedBy(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::LastUpdatedBy(this_field), Field::LastUpdatedBy(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Correlation(this_field), Field::Correlation(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::SingleReference(this_field), Field::SingleReference(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::MultiReference(this_field), Field::MultiReference(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::GridReference(this_field), Field::GridReference(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::TrackingId(this_field), Field::TrackingId(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Attachment(this_field), Field::Attachment(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::Comments(this_field), Field::Comments(other_field)) => {
                this_field.name == other_field.name
            }
            (Field::History(this_field), Field::History(other_field)) => {
                this_field.name == other_field.name
            }
            _ => false,
        }
    }
}

impl LooksLike for String {
    fn is_same_resource(&self, other: &Self) -> bool {
        self == other
    }

    fn differences(&self, _other: &Self) -> Vec<Difference> {
        vec![]
    }
}

impl LooksLike for Application {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut differences = vec![];

        // Add warning if the name is different
        push_difference!(differences, "name", &self.name, &other.name);
        push_difference!(differences, "acronym", &self.acronym, &other.acronym);
        push_difference!(differences, "disabled", &self.disabled, &other.disabled);
        push_difference!(differences, "description", &self.description, &other.description, optional: true);
        push_difference!(
            differences,
            "time_tracking_enabled",
            &self.time_tracking_enabled,
            &other.time_tracking_enabled
        );

        self.fields.iter().for_each(|field| {
            let other_field = other
                .fields
                .iter()
                .find(|other_field| field.is_same_resource(other_field));
            if let Some(other_field) = other_field {
                differences.extend(field.differences(other_field));
            } else {
                differences.push(Difference::AddingItem {
                    field: "fields".to_string(),
                    item: field.name(),
                });
            }
        });

        other.fields.iter().for_each(|field| {
            let this_field = self
                .fields
                .iter()
                .find(|this_field| field.is_same_resource(this_field));
            if this_field.is_none() {
                differences.push(Difference::RemovingItem {
                    field: "fields".to_string(),
                    item: field.name(),
                });
            }
        });

        self.workspaces.iter().for_each(|workspace| {
            let this_field = other
                .workspaces
                .iter()
                .find(|this_field| workspace.is_same_resource(this_field));
            if this_field.is_none() {
                differences.push(Difference::AddingItem {
                    field: "workspaces".to_string(),
                    item: workspace.to_owned(),
                });
            }
        });

        other.workspaces.iter().for_each(|workspace| {
            let this_field = self
                .workspaces
                .iter()
                .find(|this_field| workspace.is_same_resource(this_field));
            if this_field.is_none() {
                differences.push(Difference::RemovingItem {
                    field: "workspaces".to_string(),
                    item: workspace.to_owned(),
                });
            }
        });

        // if !self.layout.looks_like(other.layout) {
        //     differences.push(Difference::UpdatingComplexField {
        //         field: "layout".to_string(),
        //     });
        // }

        differences.extend(self.permissions.differences(&other.permissions));

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
        let source_workspace_hashmap = self.from_normaliser.get_workspace_hashmap();
        let destination_workspace_hashmap = self.to_normaliser.get_workspace_hashmap();

        // normalise the apps
        let source_apps = source_apps_future.await?;
        let destination_apps = destination_apps_future.await?;
        let source_workspace_hashmap = source_workspace_hashmap.await;
        let destination_workspace_hashmap = destination_workspace_hashmap.await;

        let source_apps = source_apps
            .into_iter()
            .map(|app| {
                self.from_normaliser
                    .normalise_application(&app, &source_workspace_hashmap)
            })
            .collect::<Vec<Application>>();

        let destination_apps = destination_apps
            .into_iter()
            .map(|app| {
                self.from_normaliser
                    .normalise_application(&app, &destination_workspace_hashmap)
            })
            .collect::<Vec<Application>>();

        self._get_resources_to_migrate(source_apps, destination_apps)
    }

    pub async fn migrate_apps(&self) -> Result<(), SwimlaneMigratorError> {
        let plans = self.get_apps_to_migrate().await?;
        for plan in &plans {
            if let MigrationPlan::Create { source_resource } = plan {
                let mut blank_app = source_resource.clone();
                blank_app.fields = vec![];
                blank_app.layout = vec![];
                self.to.create_application(&blank_app).await?;
            }
        }
        Ok(())
        // Create empty applications
        // Migrate fields
        // Migrate Tasks
        // Migrate Workflows
        // Migrate Layout
    }
}
