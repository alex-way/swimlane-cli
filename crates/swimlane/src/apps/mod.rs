pub mod fields;
pub mod layout;

use std::fmt::{Display, Formatter};

use crate::roles::PermissionMatrix;
use crate::tasks::Task;
use crate::users::UserGroupSelection;
use crate::{SwimlaneClient, SwimlaneClientError};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::fields::Field;
use self::layout::Layout;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct LightApplication {
    #[serde(rename = "$type")]
    pub _type: String,
    pub id: String,
    pub name: String,
    pub acronym: String,
    pub description: Option<String>,
    pub created_date: DateTime<Utc>,
    pub created_by_user: UserGroupSelection,
    pub modified_date: DateTime<Utc>,
    pub modified_by_user: UserGroupSelection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    #[serde(rename = "$type")]
    pub _type: String,
    pub id: String,
    pub name: String,
    pub acronym: String,
    pub description: Option<String>,
    pub tracking_field_id: String,
    pub layout: Vec<Layout>,
    pub fields: Vec<Field>,
    pub max_tracking_id: f32,
    pub workspaces: Vec<String>,
    pub create_workspace: bool,
    pub created_date: DateTime<Utc>,
    pub created_by_user: UserGroupSelection,
    pub modified_date: DateTime<Utc>,
    pub modified_by_user: UserGroupSelection,
    pub time_tracking_enabled: bool,
    pub permissions: PermissionMatrix,
    pub uid: String,
    pub version: i32,
    pub disabled: bool,
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application: {}", self.name)
    }
}

impl SwimlaneClient {
    pub async fn get_applications_light(
        &self,
    ) -> Result<Vec<LightApplication>, SwimlaneClientError> {
        let url = format!("{}/api/app/light", self.base_url);
        let applications: Vec<LightApplication> =
            self.http_client.get(url).send().await?.json().await?;
        Ok(applications)
    }

    pub async fn get_tasks_for_application(
        &self,
        application_id: &str,
    ) -> Result<Vec<Task>, SwimlaneClientError> {
        let url = format!("{}/api/task?parentId={}", self.base_url, application_id);
        let tasks: Vec<Task> = self.http_client.get(url).send().await?.json().await?;
        Ok(tasks)
    }

    pub async fn get_applications(&self) -> Result<Vec<Application>, SwimlaneClientError> {
        let url = format!("{}/api/app", self.base_url);
        let applications: Vec<Application> = self.http_client.get(url).send().await?.json().await?;
        Ok(applications)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_application_deserializes() {
        let json = r#"{
              "$type": "Core.Models.Application.ApplicationViewModel, Core",
              "id": "aMrGgbzJ9aVicl9DL",
              "name": "Alert & Incident Management",
              "acronym": "SAIM",
              "description": "",
              "createdDate": "2022-07-27T16:08:45.045Z",
              "createdByUser": {
                "$type": "Core.Models.Utilities.UserGroupSelection, Core"
              },
              "modifiedDate": "2023-03-21T14:41:52.743Z",
              "modifiedByUser": {
                "$type": "Core.Models.Utilities.UserGroupSelection, Core",
                "id": "a4UAS4FfjxrOAfrue",
                "name": "James Bond"
              }
            }"#;

        let app: LightApplication = serde_json::from_str(json).unwrap();
        assert_eq!(app.id, "aMrGgbzJ9aVicl9DL");
        assert_eq!(app.name, "Alert & Incident Management");
        assert_eq!(app.acronym, "SAIM");
        assert_eq!(app.description, Some("".to_string()));
    }

    #[test]
    fn test_application_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Application.Application, Core",
            "acronym": "AF",
            "trackingFieldId": "642af4cfebba3602af19d8d4",
            "layout": [
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "aocn0",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af4d6154ee9d8952d4421",
                "row": 1,
                "col": 1,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "afftg",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af4ef9d60d838cbac1d15",
                "row": 1,
                "col": 2,
                "sizex": 2,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "agylh",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642c1279954efe41e55c3c60",
                "row": 1,
                "col": 5,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "a7h9k",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af4f46c4a0e865cc03750",
                "row": 2,
                "col": 1,
                "sizex": 2,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "ag4kq",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af4f66e5fbc824e34cad4",
                "row": 2,
                "col": 3,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "ayl30",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af50f42427d859977f895",
                "row": 2,
                "col": 6,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "anwvk",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af511efd64a4ea8ecbb00",
                "row": 3,
                "col": 1,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "a3h0n",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af5146bc631c813f1f5ed",
                "row": 3,
                "col": 2,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "a5jd9",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af5180bda84307d2ef9d4",
                "row": 3,
                "col": 4,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "a4v6f",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af51f094c5baab311ac74",
                "row": 3,
                "col": 7,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "adgfe",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af52889f1271993c533c0",
                "row": 4,
                "col": 1,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "aan0l",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af5307ca26847f6ad9aa6",
                "row": 4,
                "col": 2,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "adz1g",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af5323f4e78fa0b893ce8",
                "row": 4,
                "col": 4,
                "sizex": 4,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "axyu8",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af5530ecc2568fa7565e0",
                "row": 5,
                "col": 1,
                "sizex": 2,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.HtmlObjectLayout, Core",
                "html": "",
                "name": "HTML",
                "layoutType": "htmlObject",
                "id": "642af832b6f331df9944b43d",
                "row": 5,
                "col": 3,
                "sizex": 2,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "aem93",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642af747581e9ca432d035e6",
                "row": 6,
                "col": 1,
                "sizex": 2,
                "sizey": 0
              },
              {
                "$type": "Core.Models.Layouts.FieldLayout, Core",
                "fieldId": "a9co5",
                "helpTextType": "none",
                "helpText": " ",
                "layoutType": "field",
                "id": "642bfd70a4dc958164481771",
                "row": 6,
                "col": 3,
                "sizex": 4,
                "sizey": 0
              }
            ],
            "fields": [
              {
                "$type": "Core.Models.Fields.ValuesListField, Core",
                "values": [
                  {
                    "$type": "Core.Models.Fields.ValuesList.ValuesListValues, Core",
                    "id": "642afdc6c0476e1ea9a0138e",
                    "name": "test2",
                    "selected": false,
                    "description": "<p>sadrsdf</p>",
                    "otherText": false,
                    "otherTextDescription": "",
                    "otherTextDefaultValue": "",
                    "otherTextRequired": "False"
                  },
                  {
                    "$type": "Core.Models.Fields.ValuesList.ValuesListValues, Core",
                    "id": "642afdbf9e2b6a5fca95f60b",
                    "name": "test",
                    "selected": true,
                    "description": "",
                    "otherText": false,
                    "otherTextDescription": "",
                    "otherTextDefaultValue": "",
                    "otherTextRequired": "False"
                  }
                ],
                "controlType": "checkbox",
                "selectionType": "multi",
                "id": "a5jd9",
                "name": "Checkboxes",
                "key": "checkboxes",
                "fieldType": "valuesList",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.CommentsField, Core",
                "id": "axyu8",
                "name": "Comments",
                "key": "comments",
                "fieldType": "comments",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.UserGroupField, Core",
                "inputType": "createdBy",
                "showAllUsers": true,
                "showAllGroups": false,
                "members": [],
                "defaults": [],
                "reverseValueMap": {
                  "$type": "System.Collections.Generic.Dictionary`2[[System.String, System.Private.CoreLib],[System.String, System.Private.CoreLib]], System.Private.CoreLib"
                },
                "controlType": "select",
                "selectionType": "single",
                "id": "aan0l",
                "name": "Created by",
                "key": "created-by",
                "fieldType": "userGroup",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.History.HistoryField, Core",
                "id": "aem93",
                "name": "History",
                "key": "history",
                "fieldType": "history",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.TextField, Core",
                "prefix": "",
                "suffix": "",
                "placeholder": "",
                "inputType": "json",
                "lengthType": "none",
                "unique": false,
                "writeOnce": false,
                "visualize": false,
                "visualizeMode": 0,
                "id": "a7h9k",
                "name": "JSON",
                "key": "json",
                "fieldType": "text",
                "required": false,
                "readOnly": true,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.UserGroupField, Core",
                "inputType": "lastUpdatedBy",
                "showAllUsers": true,
                "showAllGroups": false,
                "members": [],
                "defaults": [],
                "reverseValueMap": {
                  "$type": "System.Collections.Generic.Dictionary`2[[System.String, System.Private.CoreLib],[System.String, System.Private.CoreLib]], System.Private.CoreLib"
                },
                "controlType": "select",
                "selectionType": "single",
                "id": "adz1g",
                "name": "Last updated by",
                "key": "last-updated-by",
                "fieldType": "userGroup",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.UserGroupField, Core",
                "inputType": "userGroup",
                "showAllUsers": true,
                "showAllGroups": false,
                "members": [],
                "defaults": [],
                "reverseValueMap": {
                  "$type": "System.Collections.Generic.Dictionary`2[[System.String, System.Private.CoreLib],[System.String, System.Private.CoreLib]], System.Private.CoreLib"
                },
                "controlType": "select",
                "selectionType": "multi",
                "id": "adgfe",
                "name": "multi User/Groups",
                "key": "multi-usergroups",
                "fieldType": "userGroup",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.ValuesListField, Core",
                "values": [],
                "controlType": "select",
                "selectionType": "multi",
                "id": "anwvk",
                "name": "Multi-select",
                "key": "multi-select",
                "fieldType": "valuesList",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.ValuesListField, Core",
                "values": [],
                "controlType": "radio",
                "selectionType": "single",
                "id": "a3h0n",
                "name": "Radio buttons",
                "key": "radio-buttons",
                "fieldType": "valuesList",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.TextField, Core",
                "prefix": "",
                "suffix": "",
                "placeholder": "",
                "inputType": "rich",
                "lengthType": "words",
                "unique": false,
                "writeOnce": false,
                "visualize": false,
                "visualizeMode": 0,
                "id": "afftg",
                "name": "Rich Text",
                "key": "rich-text",
                "fieldType": "text",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.TextField, Core",
                "prefix": "",
                "suffix": "",
                "placeholder": "",
                "inputType": "text",
                "lengthType": "characters",
                "minLength": 6,
                "maxLength": 8,
                "unique": false,
                "writeOnce": false,
                "visualize": false,
                "visualizeMode": 0,
                "id": "aocn0",
                "name": "single line",
                "key": "single-line",
                "fieldType": "text",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.UserGroupField, Core",
                "inputType": "userGroup",
                "showAllUsers": false,
                "showAllGroups": false,
                "members": [
                  {
                    "$type": "Core.Models.Fields.UserGroupValues, Core",
                    "itemType": "user",
                    "selectionType": "users",
                    "id": "aN1zkqFBX2CFl0Tjy",
                    "name": "Alex Way"
                  },
                  {
                    "$type": "Core.Models.Fields.UserGroupValues, Core",
                    "itemType": "user",
                    "selectionType": "users",
                    "id": "aF0PwpA_RwT3e_rd5",
                    "name": "swadmin"
                  },
                  {
                    "$type": "Core.Models.Fields.UserGroupValues, Core",
                    "itemType": "group",
                    "selectionType": "groups",
                    "id": "aXTMhgyvjsiRY5aEz",
                    "name": "TIER-1"
                  }
                ],
                "defaults": [],
                "reverseValueMap": {
                  "$type": "System.Collections.Generic.Dictionary`2[[System.String, System.Private.CoreLib],[System.String, System.Private.CoreLib]], System.Private.CoreLib",
                  "alex Way": "aN1zkqFBX2CFl0Tjy",
                  "swadmin": "aF0PwpA_RwT3e_rd5",
                  "tieR-1": "aXTMhgyvjsiRY5aEz"
                },
                "controlType": "select",
                "selectionType": "single",
                "id": "a4v6f",
                "name": "single User/Groups",
                "key": "single-usergroups",
                "fieldType": "userGroup",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.ValuesListField, Core",
                "values": [],
                "controlType": "select",
                "selectionType": "single",
                "id": "ayl30",
                "name": "Single-select",
                "key": "single-select",
                "fieldType": "valuesList",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.List.ListField, Core",
                "supportsMultipleOutputMappings": true,
                "inputType": "text",
                "itemLengthType": "none",
                "itemStep": 0,
                "id": "ag4kq",
                "name": "Text List",
                "key": "text-list",
                "fieldType": "list",
                "required": false,
                "readOnly": false
              },
              {
                "$type": "Core.Models.Fields.List.ListField, Core",
                "supportsMultipleOutputMappings": true,
                "inputType": "text",
                "itemLengthType": "none",
                "itemStep": 0,
                "id": "agylh",
                "name": "Text List (2)",
                "key": "text-list-2",
                "fieldType": "list",
                "required": false,
                "readOnly": false
              },
              {
                "$type": "Core.Models.Fields.Date.DateField, Core",
                "inputType": "time",
                "defaultValueType": "future",
                "futurePastType": "hours",
                "futurePastValue": 0,
                "calculatedDiff": false,
                "id": "a9co5",
                "name": "Time",
                "key": "time",
                "fieldType": "date",
                "required": false,
                "readOnly": false,
                "supportsMultipleOutputMappings": false
              },
              {
                "$type": "Core.Models.Fields.TrackingField, Core",
                "prefix": "AF-",
                "id": "642af4cfebba3602af19d8d4",
                "name": "Tracking Id",
                "key": "tracking-id",
                "fieldType": "tracking",
                "readOnly": true,
                "supportsMultipleOutputMappings": false
              }
            ],
            "maxTrackingId": 0,
            "workspaces": [],
            "createWorkspace": false,
            "createdDate": "2023-04-03T15:46:23.844Z",
            "createdByUser": {
              "$type": "Core.Models.Utilities.UserGroupSelection, Core",
              "id": "aN1zkqFBX2CFl0Tjy",
              "name": "Alex Way"
            },
            "modifiedDate": "2023-04-04T13:27:11.2Z",
            "modifiedByUser": {
              "$type": "Core.Models.Utilities.UserGroupSelection, Core",
              "id": "aN1zkqFBX2CFl0Tjy",
              "name": "Alex Way"
            },
            "timeTrackingEnabled": false,
            "permissions": {
              "$type": "Core.Models.Security.PermissionMatrix, Core"
            },
            "uid": "all-the-fields-fccbb",
            "version": 39,
            "id": "aDzmDxrejKeSX3ZtE",
            "name": "All the fields",
            "disabled": false
          }"#;

        let app: Application = serde_json::from_str(json).unwrap();

        println!("{:#?}", app);
    }
}
