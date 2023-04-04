use super::constants::{
    EmailConstant, IpConstant, JsonConstant, ListConstant, MultilineConstant, RichConstant,
    TelephoneConstant, TextConstant, UrlConstant,
};
use serde::{Deserialize, Serialize};

macro_rules! core_text_field {
    ($name:ident, $input_type:ty) => {
        define_field!($name, TextConstant, {
            pub prefix: String,
            pub suffix: String,
            pub placeholder: String,
            pub input_type: $input_type,
            pub length_type: TextLengthType,
            pub unique: bool,
            pub write_once: bool,
            pub visualize: bool,
            pub visualize_mode: i64,
            pub formula: Option<String>,
        });
    };
    ($name:ident, $input_type:ty, min_max_length = true) => {
        define_field!($name, TextConstant, {
            pub prefix: String,
            pub suffix: String,
            pub placeholder: String,
            pub input_type: $input_type,
            pub length_type: TextLengthType,
            pub unique: bool,
            pub write_once: bool,
            pub visualize: bool,
            pub visualize_mode: i64,
            pub formula: Option<String>,
            pub min_length: Option<u64>,
            pub max_length: Option<u64>,
        });
    };
}

serde_enum!(TextLengthType, { None, Characters, Words });

core_text_field!(SingleLineTextField, TextConstant, min_max_length = true);
core_text_field!(MultiLineTextField, MultilineConstant, min_max_length = true);
core_text_field!(EmailField, EmailConstant);
core_text_field!(TelephoneField, TelephoneConstant);
core_text_field!(UrlField, UrlConstant);
core_text_field!(IpAddressField, IpConstant);
core_text_field!(RichTextField, RichConstant);
core_text_field!(JsonField, JsonConstant);

define_field!(TextListField, ListConstant, {
    pub input_type: TextConstant,
    pub item_length_type: TextLengthType,
    pub item_step: i64,
});

#[cfg(test)]
mod tests {
    use crate::apps::fields::constants::JsonConstant;

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

        assert_eq!(field.id, "aocn0");
        assert_eq!(field.name, "single line");
        assert_eq!(field.key, "single-line");
        assert_eq!(field.field_type, TextConstant::Text);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(!field.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, TextConstant::Text);
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

        assert_eq!(field.id, "aynwz");
        assert_eq!(field.name, "multi line");
        assert_eq!(field.key, "multi-line");
        assert_eq!(field.field_type, TextConstant::Text);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(!field.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, MultilineConstant::Multiline);
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
        assert_eq!(field.id, "apaz2");
        assert_eq!(field.name, "Email");
        assert_eq!(field.key, "email");
        assert_eq!(field.field_type, TextConstant::Text);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(!field.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, EmailConstant::Email);
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

        assert_eq!(field.id, "alixu");
        assert_eq!(field.name, "Telephone");
        assert_eq!(field.key, "telephone");
        assert_eq!(field.field_type, TextConstant::Text);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(!field.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, TelephoneConstant::Telephone);
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

        assert_eq!(field.id, "a8sms");
        assert_eq!(field.name, "URL");
        assert_eq!(field.key, "url");
        assert_eq!(field.field_type, TextConstant::Text);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(!field.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, UrlConstant::Url);
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

        assert_eq!(field.id, "avni6");
        assert_eq!(field.name, "IP");
        assert_eq!(field.key, "ip");
        assert_eq!(field.field_type, TextConstant::Text);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(!field.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, IpConstant::Ip);
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

        assert_eq!(field.id, "afftg");
        assert_eq!(field.name, "Rich Text");
        assert_eq!(field.key, "rich-text");
        assert_eq!(field.field_type, TextConstant::Text);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(!field.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, RichConstant::Rich);
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

        assert_eq!(field.id, "a7h9k");
        assert_eq!(field.name, "JSON");
        assert_eq!(field.key, "json");
        assert_eq!(field.field_type, TextConstant::Text);
        assert!(!field.required);
        assert!(field.read_only);
        assert!(!field.supports_multiple_output_mappings);
        assert_eq!(field.prefix, "");
        assert_eq!(field.suffix, "");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_type, JsonConstant::Json);
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

        assert_eq!(field.id, "ag4kq");
        assert_eq!(field.name, "Text List");
        assert_eq!(field.key, "text-list");
        assert_eq!(field.field_type, ListConstant::List);
        assert!(!field.required);
        assert!(!field.read_only);
        assert!(field.supports_multiple_output_mappings);
        assert_eq!(field.input_type, TextConstant::Text);
        assert_eq!(field.item_length_type, TextLengthType::None);
        assert_eq!(field.item_step, 0);
    }
}
