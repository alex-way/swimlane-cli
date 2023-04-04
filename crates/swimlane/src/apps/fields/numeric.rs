use serde::{Deserialize, Serialize};

use super::constants::{ListConstant, NumericConstant};

define_field!(NumericField, NumericConstant, {
    pub step: u64,
    pub unique: bool,
    pub prefix: String,
    pub suffix: String,
    pub format: String,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub formula: Option<String>,
});

define_field!(NumericListField, ListConstant, {
    pub input_type: NumericConstant,
    pub item_step: u64,
    pub item_min: Option<i64>,
    pub item_max: Option<i64>,
    pub max_items: Option<u64>,
    pub min_items: Option<u64>,
});

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
        assert_eq!(field.id, "ajy4m");
        assert_eq!(field.name, "Numeric");
        assert_eq!(field.key, "numeric");
        assert_eq!(field.field_type, NumericConstant::Numeric);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(!field.supports_multiple_output_mappings);
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
        assert_eq!(field.id, "ao7tu");
        assert_eq!(field.name, "Numeric List");
        assert_eq!(field.key, "numeric-list");
        assert_eq!(field.field_type, ListConstant::List);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(field.supports_multiple_output_mappings);
        assert_eq!(field.input_type, NumericConstant::Numeric);
        assert_eq!(field.item_step, 0);
        assert_eq!(field.item_min, None);
        assert_eq!(field.item_max, None);
        assert_eq!(field.max_items, None);
        assert_eq!(field.min_items, None);
    }
}
