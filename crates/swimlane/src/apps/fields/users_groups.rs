use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::BaseField;

serde_enum!(UserGroupInputType, { CreatedBy, LastUpdatedBy, UserGroup });
serde_enum!(UserGroupSelectionType, { Users, Groups });
serde_enum!(UserGroupItemType, { User, Group });

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupValue {
    #[serde(rename = "$type")]
    pub _type: String,
    pub item_type: UserGroupItemType,
    pub selection_type: UserGroupSelectionType,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReverseValueMap {
    #[serde(rename = "$type")]
    pub _type: String,
    /// Always "User name": "User ID" or "Group Name" : "Group ID"
    #[serde(flatten)]
    pub values: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupDefaultValue {}

serde_enum!(UserGroupFieldType, { UserGroup });

macro_rules! user_group_field {
    ($name:ident, $control_type:expr, $selection_type:expr) => {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[serde(flatten)]
            pub base: BaseField,
            pub field_type: UserGroupFieldType,
            /// Always $control_type
            pub control_type: String,
            /// Always $selection_type
            pub selection_type: String,
            pub show_all_users: bool,
            pub show_all_groups: bool,
            pub members: Vec<UserGroupValue>,
            // Defaults has an unknown value type so putting an empty struct here for now.
            pub defaults: Vec<UserGroupDefaultValue>,
            pub input_type: UserGroupInputType,
            pub reverse_value_map: ReverseValueMap,
        }
    };
}

user_group_field!(SingleUserGroupField, "user", "single");
user_group_field!(MultiUserGroupField, "user", "multiple");
user_group_field!(CreatedByField, "user", "single");
user_group_field!(LastUpdatedByField, "user", "single");
