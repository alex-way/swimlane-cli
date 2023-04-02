use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use crate::{
    roles::PermissionMatrix, users::UserGroupSelection, SwimlaneClient, SwimlaneClientError,
};

use chrono::{DateTime, Utc};
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use serde::{Deserialize, Serialize};

struct EvalTypeVisitor;

impl<'de> Visitor<'de> for EvalTypeVisitor {
    type Value = EvalType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("one of 0, 'and', 'or'")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            0 => Ok(EvalType::Zero),
            _ => Err(E::custom(format!("Invalid EvalType: {}", v))),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            "and" => Ok(EvalType::And),
            "or" => Ok(EvalType::Or),
            _ => Err(E::custom(format!("Invalid EvalType: {}", v))),
        }
    }
}

// Core.Models.Workflow.Actions.LayoutActionType
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionType {
    #[serde(rename = "showHide")]
    ShowHide,
    #[serde(rename = "readOnly")]
    ReadOnly,
    #[serde(rename = "setValue")]
    SetValue,
    #[serde(rename = "integration")]
    Integration,
    #[serde(rename = "filterValues")]
    FilterValues,
    #[serde(rename = "notification")]
    Notification,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "createNewRecord")]
    CreateNewRecord,
    #[serde(rename = "formattedExport")]
    FormattedExport,
    #[serde(rename = "toggleTimeTracking")]
    ToggleTimeTracking,
}

// Core.Models.Workflow.Actions.LayoutActionType
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LayoutActionType {
    #[serde(rename = "hide")]
    Hide,
    #[serde(rename = "show")]
    Show,
    #[serde(rename = "readOnly")]
    ReadOnly,
    #[serde(rename = "editable")]
    Editable,
    #[serde(rename = "expand")]
    Expand,
    #[serde(rename = "collapse")]
    Collapse,
}

// Core.Models.Workflow.Actions.FieldStateType
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FieldState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayoutActions {
    #[serde(rename = "$type")]
    _type: String,

    #[serde(flatten)]
    pub permissions: HashMap<String, LayoutActionType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldStates {
    #[serde(rename = "$type")]
    _type: String,

    #[serde(flatten)]
    pub permissions: HashMap<String, FieldState>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub id: String,
    #[serde(rename = "actionType")]
    pub action_type: ActionType,
    pub disabled: bool,
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "layoutActions")]
    pub layout_actions: Option<LayoutActions>,
    #[serde(rename = "fieldStates")]
    field_states: Option<FieldStates>,
}

// Core.Models.Workflow.Condition
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConditionType {
    #[serde(rename = "defaultActions")]
    DefaultActions,
    #[serde(rename = "equals")]
    Equals,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "excludes")]
    Excludes,
    #[serde(rename = "doesNotEqual")]
    DoesNotEqual,
    #[serde(rename = "lessThan")]
    LessThan,
    #[serde(rename = "lessThanOrEqual")]
    LessThanOrEqual,
    #[serde(rename = "greaterThan")]
    GreaterThan,
    #[serde(rename = "greaterThanOrEqual")]
    GreaterThanOrEqual,
    #[serde(rename = "regex")]
    Regex,
    #[serde(rename = "hasValue")]
    HasValue,
    #[serde(rename = "doesNotHaveValue")]
    DoesNotHaveValue,
    #[serde(rename = "hasBeenModified")]
    HasBeenModified,
}

#[derive(Serialize, Debug, Clone)]
pub enum EvalType {
    Zero,
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
}

impl<'de> Deserialize<'de> for EvalType {
    fn deserialize<D>(deserializer: D) -> Result<EvalType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(EvalTypeVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
    #[serde(rename = "conditionType")]
    pub condition_type: String,
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "referenceFieldConjunction")]
    pub reference_field_conjunction: EvalType,
    // pub value: Option<String>, // can also be i32 or struct...
    #[serde(rename = "isCaseSensitive")]
    pub is_case_sensitive: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RepeatFilterType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "equals")]
    Excludes,
    #[serde(rename = "doesNotEqual")]
    GreaterThanOrEqual,
    #[serde(rename = "regex")]
    Regex,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repeat {
    pub id: String,
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "fieldId")]
    pub field_id: String,
    pub actions: Vec<Action>,
    pub disabled: bool,
    #[serde(rename = "filterType")]
    pub filter_type: RepeatFilterType,
    #[serde(rename = "filterValue")]
    pub filter_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stage {
    pub id: String,
    pub name: String,
    pub disabled: bool,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "conditionType")]
    pub condition_type: Option<ConditionType>,
    #[serde(rename = "evalType")]
    pub eval_type: Option<EvalType>,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub repeats: Vec<Repeat>,
    pub stages: Vec<Stage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workflow {
    pub id: String,
    #[serde(rename = "applicationId")]
    pub application_id: String,
    pub version: i32,
    pub disabled: bool,
    pub uid: String,
    #[serde(rename = "createdByUser")]
    pub created_by_user: Option<UserGroupSelection>,
    #[serde(rename = "modifiedByUser")]
    pub modified_by_user: Option<UserGroupSelection>,
    #[serde(rename = "createdDate")]
    pub created_date: DateTime<Utc>,
    #[serde(rename = "modifiedDate")]
    pub modified_date: DateTime<Utc>,
    pub permissions: PermissionMatrix,
    pub stages: Vec<Stage>,
}

impl Display for Workflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Workflow: {}", self.application_id)
    }
}

impl SwimlaneClient {
    pub async fn get_workflows(&self) -> Result<Vec<Workflow>, SwimlaneClientError> {
        let url = format!("{}/api/workflow/", self.base_url);
        let workflows: Vec<Workflow> = self.http_client.get(url).send().await?.json().await?;
        Ok(workflows)
    }
}
