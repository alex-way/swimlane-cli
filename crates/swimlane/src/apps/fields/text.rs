use serde::{Deserialize, Serialize};

use super::BaseField;

serde_enum!(TextLengthType, { None, Characters, Words });

serde_enum!(TextFieldType, { Text, List });

macro_rules! core_text_field {
    ($name:ident, $input_type:expr) => {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[serde(flatten)]
            pub base: BaseField,
            pub field_type: TextFieldType,
            pub prefix: String,
            pub suffix: String,
            pub placeholder: String,
            pub input_type: String,
            pub length_type: TextLengthType,
            pub unique: bool,
            pub write_once: bool,
            pub visualize: bool,
            pub visualize_mode: i64,
            pub formula: Option<String>,
        }
    };
    ($name:ident, $input_type:expr, min_max_length = true) => {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            #[serde(flatten)]
            pub base: BaseField,
            pub field_type: TextFieldType,
            pub prefix: String,
            pub suffix: String,
            pub placeholder: String,
            pub input_type: String,
            pub length_type: TextLengthType,
            pub unique: bool,
            pub write_once: bool,
            pub visualize: bool,
            pub visualize_mode: i64,
            pub formula: Option<String>,
            pub min_length: Option<u64>,
            pub max_length: Option<u64>,
        }
    };
}

core_text_field!(SingleLineTextField, "text", min_max_length = true);
core_text_field!(MultiLineTextField, "multiline", min_max_length = true);
core_text_field!(EmailField, "email");
core_text_field!(TelephoneField, "telephone");
core_text_field!(UrlField, "url");
core_text_field!(IpAddressField, "ip");
core_text_field!(RichTextField, "rich");
core_text_field!(JsonField, "json");

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct TextListField {
    #[serde(flatten)]
    pub base: BaseField,
    pub field_type: TextFieldType,
    pub input_type: String,
    pub item_length_type: TextLengthType,
    pub item_step: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_single_line_text_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.TextField, Core",
            "prefix": "",
            "suffix": "",
            "placeholder": "",
            "inputType": "text",
            "lengthType": "none",
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
        }"#;

        let field: SingleLineTextField = serde_json::from_str(json).unwrap();

        assert_eq!(field.base.id, "aocn0");
        assert_eq!(field.base.name, "single line");
        assert_eq!(field.base.key, "single-line");
        assert_eq!(field.field_type, TextFieldType::Text);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, "text");
        assert_eq!(field.length_type, TextLengthType::None);
        assert!(!field.unique);
        assert!(!field.write_once);
        assert!(!field.visualize);
        assert_eq!(field.visualize_mode, 0);
    }

    #[test]
    fn test_default_multi_line_text_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.TextField, Core",
            "prefix": "",
            "suffix": "",
            "placeholder": "",
            "inputType": "multiline",
            "lengthType": "none",
            "unique": false,
            "writeOnce": false,
            "visualize": false,
            "visualizeMode": 0,
            "id": "aynwz",
            "name": "multi line",
            "key": "multi-line",
            "fieldType": "text",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
        }"#;

        let field: MultiLineTextField = serde_json::from_str(json).unwrap();

        assert_eq!(field.base.id, "aynwz");
        assert_eq!(field.base.name, "multi line");
        assert_eq!(field.base.key, "multi-line");
        assert_eq!(field.field_type, TextFieldType::Text);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, "multiline");
        assert_eq!(field.length_type, TextLengthType::None);
        assert!(!field.unique);
        assert!(!field.write_once);
        assert!(!field.visualize);
        assert_eq!(field.visualize_mode, 0);
    }

    #[test]
    fn test_default_email_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.TextField, Core",
            "prefix": "",
            "suffix": "",
            "placeholder": "",
            "inputType": "email",
            "lengthType": "none",
            "unique": false,
            "writeOnce": false,
            "visualize": false,
            "visualizeMode": 0,
            "id": "apaz2",
            "name": "Email",
            "key": "email",
            "fieldType": "text",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
        }"#;

        let field: EmailField = serde_json::from_str(json).unwrap();
        assert_eq!(field.base.id, "apaz2");
        assert_eq!(field.base.name, "Email");
        assert_eq!(field.base.key, "email");
        assert_eq!(field.field_type, TextFieldType::Text);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, "email");
        assert_eq!(field.length_type, TextLengthType::None);
        assert!(!field.unique);
        assert!(!field.write_once);
        assert!(!field.visualize);
        assert_eq!(field.visualize_mode, 0);
    }

    #[test]
    fn test_default_telephone_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.TextField, Core",
            "prefix": "",
            "suffix": "",
            "placeholder": "",
            "inputType": "telephone",
            "lengthType": "none",
            "unique": false,
            "writeOnce": false,
            "visualize": false,
            "visualizeMode": 0,
            "id": "alixu",
            "name": "Telephone",
            "key": "telephone",
            "fieldType": "text",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
        }"#;

        let field: TelephoneField = serde_json::from_str(json).unwrap();

        assert_eq!(field.base.id, "alixu");
        assert_eq!(field.base.name, "Telephone");
        assert_eq!(field.base.key, "telephone");
        assert_eq!(field.field_type, TextFieldType::Text);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, "telephone");
        assert_eq!(field.length_type, TextLengthType::None);
        assert!(!field.unique);
        assert!(!field.write_once);
        assert!(!field.visualize);
        assert_eq!(field.visualize_mode, 0);
    }

    #[test]
    fn test_default_url_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.TextField, Core",
            "prefix": "",
            "suffix": "",
            "placeholder": "",
            "inputType": "url",
            "lengthType": "none",
            "unique": false,
            "writeOnce": false,
            "visualize": false,
            "visualizeMode": 0,
            "id": "a8sms",
            "name": "URL",
            "key": "url",
            "fieldType": "text",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
        }"#;

        let field: UrlField = serde_json::from_str(json).unwrap();

        assert_eq!(field.base.id, "a8sms");
        assert_eq!(field.base.name, "URL");
        assert_eq!(field.base.key, "url");
        assert_eq!(field.field_type, TextFieldType::Text);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, "url");
        assert_eq!(field.length_type, TextLengthType::None);
        assert!(!field.unique);
        assert!(!field.write_once);
        assert!(!field.visualize);
        assert_eq!(field.visualize_mode, 0);
    }

    #[test]
    fn test_default_ip_address_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.TextField, Core",
            "prefix": "",
            "suffix": "",
            "placeholder": "",
            "inputType": "ip",
            "lengthType": "none",
            "unique": false,
            "writeOnce": false,
            "visualize": false,
            "visualizeMode": 0,
            "id": "avni6",
            "name": "IP",
            "key": "ip",
            "fieldType": "text",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
        }"#;

        let field: IpAddressField = serde_json::from_str(json).unwrap();

        assert_eq!(field.base.id, "avni6");
        assert_eq!(field.base.name, "IP");
        assert_eq!(field.base.key, "ip");
        assert_eq!(field.field_type, TextFieldType::Text);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, "ip");
        assert_eq!(field.length_type, TextLengthType::None);
        assert!(!field.unique);
        assert!(!field.write_once);
        assert!(!field.visualize);
        assert_eq!(field.visualize_mode, 0);
    }

    #[test]
    fn test_default_rich_text_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.TextField, Core",
            "prefix": "",
            "suffix": "",
            "placeholder": "",
            "inputType": "rich",
            "lengthType": "none",
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
        }"#;

        let field: RichTextField = serde_json::from_str(json).unwrap();

        assert_eq!(field.base.id, "afftg");
        assert_eq!(field.base.name, "Rich Text");
        assert_eq!(field.base.key, "rich-text");
        assert_eq!(field.field_type, TextFieldType::Text);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, "rich");
        assert_eq!(field.length_type, TextLengthType::None);
        assert!(!field.unique);
        assert!(!field.write_once);
        assert!(!field.visualize);
        assert_eq!(field.visualize_mode, 0);
    }

    #[test]
    fn test_default_json_field_deserializes() {
        let json = r#"{
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
        }"#;

        let field: JsonField = serde_json::from_str(json).unwrap();

        assert_eq!(field.base.id, "a7h9k");
        assert_eq!(field.base.name, "JSON");
        assert_eq!(field.base.key, "json");
        assert_eq!(field.field_type, TextFieldType::Text);
        assert!(!field.base.required);
        assert!(field.base.read_only);
        assert!(!field.base.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, "json");
        assert_eq!(field.length_type, TextLengthType::None);
        assert!(!field.unique);
        assert!(!field.write_once);
        assert!(!field.visualize);
        assert_eq!(field.visualize_mode, 0);
    }

    #[test]
    fn test_default_text_list_field_deserializes() {
        let json = r#"{
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
        }"#;

        let field: TextListField = serde_json::from_str(json).unwrap();

        assert_eq!(field.base.id, "ag4kq");
        assert_eq!(field.base.name, "Text List");
        assert_eq!(field.base.key, "text-list");
        assert_eq!(field.field_type, TextFieldType::List);
        assert!(!field.base.required);
        assert!(!field.base.read_only);
        assert!(field.base.supports_multiple_output_mappings);
        assert_eq!(field.input_type, "text");
        assert_eq!(field.item_length_type, TextLengthType::None);
        assert_eq!(field.item_step, 0);
    }
}
