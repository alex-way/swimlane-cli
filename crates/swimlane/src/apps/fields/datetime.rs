use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{BaseField, FieldType};

/// Used for DateField to specify the minMode field
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DateFieldMinMode {
    Day,
    Month,
    Year,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DateTimeFieldFuturePastType {
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

/// Used for DateField to specify the defaultValueType field
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DateTimeFieldDefaultValueType {
    Future,
    Specific,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct DateField {
    #[serde(flatten)]
    pub base: BaseField,
    pub field_type: FieldType,
    /// Always "date"
    pub input_type: String,
    // Always "date"
    // pub field_type: String,
    pub min_mode: DateFieldMinMode,
    pub default_value_type: DateTimeFieldDefaultValueType,
    pub default_value: Option<DateTime<Utc>>,
    pub future_past_type: DateTimeFieldFuturePastType,
    pub future_past_value: i64,
    pub calculated_diff: bool,
    pub formula: Option<String>,
}

/// Used for TimeField to specify the futurePastType field
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TimeFieldFuturePastType {
    Minutes,
    Hours,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct TimeField {
    #[serde(flatten)]
    pub base: BaseField,
    pub field_type: FieldType,
    /// Always "time"
    pub input_type: String,
    // Always "date"
    // pub field_type: String,
    pub default_value_type: DateTimeFieldDefaultValueType,
    /// ISO 8601 datetime for 1970-01-01T00:00:00Z. e.g. "1970-01-01T13:30:00Z"
    pub default_value: Option<DateTime<Utc>>,
    pub future_past_type: TimeFieldFuturePastType,
    pub future_past_value: i64,
    pub calculated_diff: bool,
    pub formula: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeField {
    #[serde(flatten)]
    pub base: BaseField,
    pub field_type: FieldType,
    /// Always "dateTime"
    pub input_type: String,
    // Always "date"
    // pub field_type: String,
    pub default_value_type: DateTimeFieldDefaultValueType,
    /// ISO 8601 datetime for 1970-01-01T00:00:00Z. e.g. "1970-01-01T13:30:00Z"
    pub default_value: Option<DateTime<Utc>>,
    pub future_past_type: DateTimeFieldFuturePastType,
    pub future_past_value: i64,
    pub calculated_diff: bool,
    pub formula: Option<String>,
}

/// Used for TimeSpanField to specify the start and end fields
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeSpan {
    #[serde(rename = "timeSpanDiffStartField")]
    pub start_field: String,
    #[serde(rename = "timeSpanDiffEndField")]
    pub end_field: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct TimeSpanField {
    #[serde(flatten)]
    pub base: BaseField,
    pub field_type: FieldType,
    /// Always "timespan"
    pub input_type: String,
    pub default_value_type: String,
    pub future_past_value: i64,
    pub calculated_diff: bool,
    // Always "date"
    // pub field_type: String,
    #[serde(flatten)]
    pub time_span: Option<TimeSpan>,
    pub formula: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FirstCreatedLastUpdatedDefaultValueType {
    None,
}

// A simple macro to encapsulate the "First Created" and "Last Updated" fields in Swimlane.
macro_rules! builtin_date_field {
    ($name:ident, $input_type:expr, $field_type:expr) => {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[serde(flatten)]
            pub base: BaseField,
            pub field_type: FieldType,
            /// Always "firstCreated"
            pub input_type: String,
            // Always "date"
            // pub field_type: String,
            pub default_value_type: FirstCreatedLastUpdatedDefaultValueType,
            pub future_past_value: i64,
            pub calculated_diff: bool,
        }
    };
}

builtin_date_field!(FirstCreatedField, "firstCreated", "date");
builtin_date_field!(LastUpdatedField, "lastUpdated", "date");

#[cfg(test)]
mod tests {
    use crate::apps::fields::FieldType;

    use super::*;

    #[test]
    fn test_default_first_created_field_deserializes() {
        let json = r#"{
			"$type": "Core.Models.Fields.Date.DateField, Core",
			"inputType": "firstCreated",
			"defaultValueType": "none",
			"futurePastValue": 0,
			"calculatedDiff": false,
			"id": "awohj",
			"name": "First Created",
			"key": "first-created",
			"fieldType": "date",
			"required": false,
			"readOnly": false,
			"supportsMultipleOutputMappings": false
		}"#;

        let field: FirstCreatedField = serde_json::from_str(json).unwrap();

        assert_eq!(field.base.id, "awohj");
        assert_eq!(field.base.name, "First Created");
        assert_eq!(field.base.key, "first-created");
        assert_eq!(field.field_type, FieldType::Date);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.input_type, "firstCreated");
        assert_eq!(
            field.default_value_type,
            FirstCreatedLastUpdatedDefaultValueType::None
        );
        assert_eq!(field.future_past_value, 0);
        assert!(!field.calculated_diff);
    }

    #[test]
    fn test_default_last_updated_field_deserializes() {
        let json = r#"{
			"$type": "Core.Models.Fields.Date.DateField, Core",
			"inputType": "lastUpdated",
			"defaultValueType": "none",
			"futurePastValue": 0,
			"calculatedDiff": false,
			"id": "adb8i",
			"name": "Last Updated",
			"key": "last-updated",
			"fieldType": "date",
			"required": false,
			"readOnly": false,
			"supportsMultipleOutputMappings": false
		}"#;
        let field: LastUpdatedField = serde_json::from_str(json).unwrap();
        assert_eq!(field.base.id, "adb8i");
        assert_eq!(field.base.name, "Last Updated");
        assert_eq!(field.base.key, "last-updated");
        assert_eq!(field.field_type, FieldType::Date);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.input_type, "lastUpdated");
        assert_eq!(
            field.default_value_type,
            FirstCreatedLastUpdatedDefaultValueType::None
        );
        assert_eq!(field.future_past_value, 0);
        assert!(!field.calculated_diff);
    }
}
