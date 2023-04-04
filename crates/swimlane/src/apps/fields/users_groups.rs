use std::collections::HashMap;

use super::constants::{MultiConstant, SelectConstant, SingleConstant, UserGroupConstant};
use serde::{Deserialize, Serialize};

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

macro_rules! user_group_field {
    ($name:ident, $selection_type:ty) => {
        define_field!($name, UserGroupConstant, {
            pub control_type: SelectConstant,
            pub selection_type: $selection_type,
            pub show_all_users: bool,
            pub show_all_groups: bool,
            pub members: Vec<UserGroupValue>,
            // Defaults has an unknown value type so putting an empty struct here for now.
            pub defaults: Vec<UserGroupDefaultValue>,
            pub input_type: UserGroupInputType,
            pub reverse_value_map: ReverseValueMap,
        });
    };
}

user_group_field!(SingleUserGroupField, SingleConstant);
user_group_field!(MultiUserGroupField, MultiConstant);
user_group_field!(CreatedByField, SingleConstant);
user_group_field!(LastUpdatedByField, SingleConstant);
