use serde::{Deserialize, Serialize};

use super::{BaseField, FieldType};

macro_rules! reference_field {
    ($name:ident, $control_type:expr, $selection_type:expr, $field_type:expr) => {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[serde(flatten)]
            pub base: BaseField,
            pub field_type: FieldType,
            pub target_id: String,
            pub columns: Vec<String>,
            /// Always $control_type
            pub control_type: String,
            /// Always $selection_type
            pub selection_type: String,
            pub can_add: bool,
            pub create_backreference: bool,
            // Always $field_type
            // pub field_type: String,
        }
    };
}

reference_field!(SingleReferenceField, "select", "single", "reference");
reference_field!(MultiReferenceField, "select", "multi", "reference");
reference_field!(GridReferenceField, "select", "multi", "reference");
reference_field!(CorrelationField, "correlation", "single", "reference");

#[cfg(test)]
mod tests {
    use crate::apps::fields::FieldType;

    use super::*;

    #[test]
    fn test_default_single_reference_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.Reference.ReferenceField, Core",
            "targetId": "aX3p1GZUvHfd9t7mg",
            "columns": [
              "62e3851e6933ba02ad266b69"
            ],
            "controlType": "select",
            "selectionType": "single",
            "canAdd": true,
            "createBackreference": false,
            "id": "asfyh",
            "name": "single Reference",
            "key": "single-reference",
            "fieldType": "reference",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
          }"#;

        let field: SingleReferenceField = serde_json::from_str(json).unwrap();
        assert_eq!(field.base.id, "asfyh");
        assert_eq!(field.base.name, "single Reference");
        assert_eq!(field.base.key, "single-reference");
        assert_eq!(field.field_type, FieldType::Reference);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.target_id, "aX3p1GZUvHfd9t7mg");
        assert_eq!(field.columns, vec!["62e3851e6933ba02ad266b69"]);
        assert_eq!(field.control_type, "select");
        assert_eq!(field.selection_type, "single");
        assert!(field.can_add);
        assert!(!field.create_backreference);
    }

    #[test]
    fn test_default_multi_reference_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.Reference.ReferenceField, Core",
            "targetId": "aX3p1GZUvHfd9t7mg",
            "columns": [
              "62e3851e6933ba02ad266b69"
            ],
            "controlType": "select",
            "selectionType": "multi",
            "canAdd": true,
            "createBackreference": false,
            "id": "ads1h",
            "name": "multi Reference",
            "key": "multi-reference",
            "fieldType": "reference",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
          }"#;
        let field: MultiReferenceField = serde_json::from_str(json).unwrap();
        assert_eq!(field.base.id, "ads1h");
        assert_eq!(field.base.name, "multi Reference");
        assert_eq!(field.base.key, "multi-reference");
        assert_eq!(field.field_type, FieldType::Reference);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.target_id, "aX3p1GZUvHfd9t7mg");
        assert_eq!(field.columns, vec!["62e3851e6933ba02ad266b69"]);
        assert_eq!(field.control_type, "select");
        assert_eq!(field.selection_type, "multi");
        assert!(field.can_add);
        assert!(!field.create_backreference);
    }

    #[test]
    fn test_default_grid_reference_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.Reference.ReferenceField, Core",
            "targetId": "aX3p1GZUvHfd9t7mg",
            "columns": [
              "62e3851e6933ba02ad266b69"
            ],
            "controlType": "grid",
            "selectionType": "multi",
            "canAdd": true,
            "createBackreference": false,
            "id": "a9tiq",
            "name": "grid Reference",
            "key": "grid-reference",
            "fieldType": "reference",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
          }"#;
        let field: GridReferenceField = serde_json::from_str(json).unwrap();
        assert_eq!(field.base.id, "a9tiq");
        assert_eq!(field.base.name, "grid Reference");
        assert_eq!(field.base.key, "grid-reference");
        assert_eq!(field.field_type, FieldType::Reference);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.target_id, "aX3p1GZUvHfd9t7mg");
        assert_eq!(field.columns, vec!["62e3851e6933ba02ad266b69"]);
        assert_eq!(field.control_type, "grid");
        assert_eq!(field.selection_type, "multi");
        assert!(field.can_add);
        assert!(!field.create_backreference);
    }

    #[test]
    fn test_default_correlation_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.Reference.ReferenceField, Core",
            "targetId": "aDzmDxrejKeSX3ZtE",
            "columns": [
              "642af4cfebba3602af19d8d4"
            ],
            "controlType": "correlation",
            "selectionType": "single",
            "canAdd": false,
            "createBackreference": false,
            "id": "avwdh",
            "name": "Correlation",
            "key": "correlation",
            "fieldType": "reference",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
          }"#;
        let field: CorrelationField = serde_json::from_str(json).unwrap();
        assert_eq!(field.base.id, "avwdh");
        assert_eq!(field.base.name, "Correlation");
        assert_eq!(field.base.key, "correlation");
        assert_eq!(field.field_type, FieldType::Reference);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.target_id, "aDzmDxrejKeSX3ZtE");
        assert_eq!(field.columns, vec!["642af4cfebba3602af19d8d4"]);
        assert_eq!(field.control_type, "correlation");
        assert_eq!(field.selection_type, "single");
        assert!(!field.can_add);
        assert!(!field.create_backreference);
    }
}
