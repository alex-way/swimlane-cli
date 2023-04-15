use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::constants::{
    DateConstant, DateTimeConstant, FirstCreatedConstant, LastUpdatedConstant, TimeConstant,
    TimespanConstant,
};

// Used for DateField to specify the minMode field
serde_enum!(DateFieldMinMode, { Day, Month, Year });
serde_enum!(DateTimeFieldFuturePastType, { Minutes, Hours, Days, Months, Years });

// Used for DateField to specify the defaultValueType field
serde_enum!(DateTimeFieldDefaultValueType, { None, Future, Specific, Current });

define_field!(DateField, DateConstant, {
    pub input_type: DateConstant,
    pub min_mode: Option<DateFieldMinMode>,
    pub default_value_type: DateTimeFieldDefaultValueType,
    pub default_value: Option<DateTime<Utc>>,
    pub future_past_type: Option<DateTimeFieldFuturePastType>,
    pub future_past_value: i64,
    pub calculated_diff: bool,
    pub formula: Option<String>,
});

// Used for TimeField to specify the futurePastType field
serde_enum!(TimeFieldFuturePastType, { Minutes, Hours });

define_field!(TimeField, DateConstant, {
    pub input_type: TimeConstant,
    pub default_value_type: DateTimeFieldDefaultValueType,
    /// ISO 8601 datetime for 1970-01-01T00:00:00Z. e.g. "1970-01-01T13:30:00Z"
    pub default_value: Option<DateTime<Utc>>,
    pub future_past_type: Option<TimeFieldFuturePastType>,
    pub future_past_value: i64,
    pub calculated_diff: bool,
    pub formula: Option<String>,
});

define_field!(DateTimeField, DateConstant, {
    pub input_type: DateTimeConstant,
    pub default_value_type: DateTimeFieldDefaultValueType,
    /// ISO 8601 datetime for 1970-01-01T00:00:00Z. e.g. "1970-01-01T13:30:00Z"
    pub default_value: Option<DateTime<Utc>>,
    pub future_past_type: Option<DateTimeFieldFuturePastType>,
    pub future_past_value: i64,
    pub calculated_diff: bool,
    pub formula: Option<String>,
});

define_field!(TimeSpanField, DateConstant, {
    pub input_type: TimespanConstant,
    pub default_value_type: String,
    pub future_past_value: i64,
    pub calculated_diff: bool,
    pub time_span_diff_start_field: String,
    pub time_span_diff_end_field: String,
    pub formula: Option<String>,
});

serde_enum!(FirstCreatedLastUpdatedDefaultValueType, { None });

// A simple macro to encapsulate the "First Created" and "Last Updated" fields in Swimlane.
macro_rules! builtin_date_field {
    ($name:ident, $input_type:ty) => {
        define_field!($name, DateConstant, {
            pub input_type: $input_type,
            pub default_value_type: FirstCreatedLastUpdatedDefaultValueType,
            pub future_past_value: i64,
            pub calculated_diff: bool,
        });
    };
}

builtin_date_field!(FirstCreatedField, FirstCreatedConstant);
builtin_date_field!(LastUpdatedField, LastUpdatedConstant);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_field_deserializes() {
        let json = r#"{
            "$type":"Core.Models.Fields.Date.DateField, Core",
            "inputType":"dateTime",
            "defaultValueType":"none",
            "futurePastType":"hours",
            "futurePastValue":0,
            "calculatedDiff":false,
            "id":"aPDaSsV9gpGyS0Ki5",
            "name":"Last Teams Message Sent Timestamp",
            "key":"last-teams-msg-sent-timestamp",
            "fieldType":"date",
            "required":false,
            "readOnly":false,
            "supportsMultipleOutputMappings":false
        }"#;

        let _field: DateTimeField = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_date_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.Date.DateField, Core",
            "inputType": "date",
            "defaultValueType": "none",
            "futurePastType": "hours",
            "futurePastValue": 0,
            "calculatedDiff": false,
            "id": "a65un",
            "name": "End Date",
            "key": "end-date",
            "fieldType": "date",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
          }"#;

        let _field: DateField = serde_json::from_str(json).unwrap();
    }

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

        assert_eq!(field.id, "awohj");
        assert_eq!(field.name, "First Created");
        assert_eq!(field.key, "first-created");
        assert_eq!(field.field_type, DateConstant::Date);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(!field.supports_multiple_output_mappings);
        assert_eq!(field.input_type, FirstCreatedConstant::FirstCreated);
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
        assert_eq!(field.id, "adb8i");
        assert_eq!(field.name, "Last Updated");
        assert_eq!(field.key, "last-updated");
        assert_eq!(field.field_type, DateConstant::Date);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(!field.supports_multiple_output_mappings);
        assert_eq!(field.input_type, LastUpdatedConstant::LastUpdated);
        assert_eq!(
            field.default_value_type,
            FirstCreatedLastUpdatedDefaultValueType::None
        );
        assert_eq!(field.future_past_value, 0);
        assert!(!field.calculated_diff);
    }
}
