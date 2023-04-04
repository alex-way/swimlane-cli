use serde::{Deserialize, Serialize};

use super::BaseField;

serde_enum!(NumericFieldType, { Numeric, List });

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct NumericField {
    #[serde(flatten)]
    pub base: BaseField,
    pub field_type: NumericFieldType,
    /// Always 1?
    pub step: u64,
    pub unique: bool,
    pub prefix: String,
    pub suffix: String,
    /// Always "none"?
    pub format: String,
    pub min: Option<i64>,
    pub max: Option<i64>,
    /// Always "numeric"
    // pub field_type: String,
    pub formula: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct NumericListField {
    #[serde(flatten)]
    pub base: BaseField,
    pub field_type: NumericFieldType,
    /// Always "numeric"
    pub input_type: String,
    // Always "list"
    // pub field_type: String,
    /// Always 0?
    pub item_step: u64,
    pub item_min: Option<i64>,
    pub item_max: Option<i64>,
    pub max_items: Option<u64>,
    pub min_items: Option<u64>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_default_numeric_field_deserializes() {
        let json = r#"{
			"$type": "Core.Models.Fields.NumericField, Core",
			"step": 1,
			"unique": false,
			"prefix": "",
			"suffix": "",
			"format": "none",
			"id": "ajy4m",
			"name": "Numeric",
			"key": "numeric",
			"fieldType": "numeric",
			"required": false,
			"readOnly": false,
			"supportsMultipleOutputMappings": false
		}"#;

        let field: NumericField = serde_json::from_str(json).unwrap();
        assert_eq!(field.base.id, "ajy4m");
        assert_eq!(field.base.name, "Numeric");
        assert_eq!(field.base.key, "numeric");
        assert_eq!(field.field_type, NumericFieldType::Numeric);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.step, 1);
        assert!(!field.unique);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.format, "none");
        assert_eq!(field.min, None);
        assert_eq!(field.max, None);
        assert_eq!(field.formula, None);
    }

    #[test]
    fn test_default_numeric_list_field_deserializes() {
        let json = r#"{
			"$type": "Core.Models.Fields.List.ListField, Core",
			"supportsMultipleOutputMappings": true,
			"inputType": "numeric",
			"itemStep": 0,
			"id": "ao7tu",
			"name": "Numeric List",
			"key": "numeric-list",
			"fieldType": "list",
			"required": false,
			"readOnly": false
		}"#;
        let field: NumericListField = serde_json::from_str(json).unwrap();
        assert_eq!(field.base.id, "ao7tu");
        assert_eq!(field.base.name, "Numeric List");
        assert_eq!(field.base.key, "numeric-list");
        assert_eq!(field.field_type, NumericFieldType::List);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(field.base.supports_multiple_output_mappings);
        assert_eq!(field.input_type, "numeric");
        assert_eq!(field.item_step, 0);
        assert_eq!(field.item_min, None);
        assert_eq!(field.item_max, None);
        assert_eq!(field.max_items, None);
        assert_eq!(field.min_items, None);
    }
}
