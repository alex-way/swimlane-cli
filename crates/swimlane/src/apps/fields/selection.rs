use super::constants::{
    CheckboxConstant, MultiConstant, RadioConstant, SelectConstant, SingleConstant,
    ValuesListConstant,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ValuesListValue {
    #[serde(rename = "$type")]
    pub _type: String,
    pub id: String,
    pub name: String,
    pub selected: bool,
    pub description: String,
    pub other_text: bool,
    pub other_text_description: String,
    pub other_text_default_value: String,
    pub other_text_required: String,
}

macro_rules! selection_field {
    ($name:ident, $control_type:ty, $selection_type:ty) => {
      define_field!($name, ValuesListConstant, {
            pub values: Vec<ValuesListValue>,
            pub control_type: $control_type,
            pub selection_type: $selection_type,
        });
    };
}

selection_field!(SingleSelectField, SelectConstant, SingleConstant);
selection_field!(MultiSelectField, SelectConstant, MultiConstant);
selection_field!(RadioButtonsField, RadioConstant, SingleConstant);
selection_field!(CheckboxesField, CheckboxConstant, MultiConstant);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_single_select_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.Selection.SelectionField, Core",
            "values": [
              {
                "$type": "Core.Models.Fields.ValuesList.ValuesListValues, Core",
                "id": "1",
                "name": "Value 1",
                "selected": false,
                "description": "",
                "otherText": false,
                "otherTextDescription": "",
                "otherTextDefaultValue": "",
                "otherTextRequired": ""
              },
              {
                "$type": "Core.Models.Fields.ValuesList.ValuesListValues, Core",
                "id": "2",
                "name": "Value 2",
                "selected": false,
                "description": "",
                "otherText": false,
                "otherTextDescription": "",
                "otherTextDefaultValue": "",
                "otherTextRequired": ""
              }
            ],
            "controlType": "select",
            "selectionType": "single",
            "id": "asfyh",
            "name": "single Select",
            "key": "single-select",
            "fieldType": "valuesList",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
          }"#;

        let field: SingleSelectField = serde_json::from_str(json).unwrap();

        assert_eq!(field.id, "asfyh");
        assert_eq!(field.name, "single Select");
        assert_eq!(field.key, "single-select");
        assert_eq!(field.field_type, ValuesListConstant::ValuesList);
        assert_eq!(field.values.len(), 2);
        assert_eq!(field.values[0].id, "1");
        assert_eq!(field.values[0].name, "Value 1");
        assert!(!field.values[0].selected);
        assert_eq!(field.values[0].description, "");
        assert!(!field.values[0].other_text);
        assert_eq!(field.values[0].other_text_description, "");
        assert_eq!(field.values[0].other_text_default_value, "");
        assert_eq!(field.values[0].other_text_required, "");
        assert_eq!(field.values[1].id, "2");
        assert_eq!(field.values[1].name, "Value 2");
        assert!(!field.values[1].selected);
        assert_eq!(field.values[1].description, "");
        assert!(!field.values[1].other_text);
        assert_eq!(field.values[1].other_text_description, "");
    }

    #[test]
    fn test_multi_select_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.Selection.SelectionField, Core",
            "values": [
              {
                "$type": "Core.Models.Fields.ValuesList.ValuesListValues, Core",
                "id": "1",
                "name": "Value 1",
                "selected": false,
                "description": "",
                "otherText": false,
                "otherTextDescription": "",
                "otherTextDefaultValue": "",
                "otherTextRequired": ""
              },
              {
                "$type": "Core.Models.Fields.ValuesList.ValuesListValues, Core",
                "id": "2",
                "name": "Value 2",
                "selected": false,
                "description": "",
                "otherText": false,
                "otherTextDescription": "",
                "otherTextDefaultValue": "",
                "otherTextRequired": ""
              }
            ],
            "controlType": "select",
            "selectionType": "multi",
            "id": "asfyh",
            "name": "multi Select",
            "key": "multi-select",
            "fieldType": "valuesList",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
          }"#;

        let field: MultiSelectField = serde_json::from_str(json).unwrap();

        assert_eq!(field.id, "asfyh");
        assert_eq!(field.name, "multi Select");
        assert_eq!(field.key, "multi-select");
        assert_eq!(field.field_type, ValuesListConstant::ValuesList);
        assert_eq!(field.values.len(), 2);
        assert_eq!(field.values[0].id, "1");
        assert_eq!(field.values[0].name, "Value 1");
        assert!(!field.values[0].selected);
        assert_eq!(field.values[0].description, "");
        assert!(!field.values[0].other_text);
        assert_eq!(field.values[0].other_text_description, "");
        assert_eq!(field.values[0].other_text_default_value, "");
        assert_eq!(field.values[0].other_text_required, "");
        assert_eq!(field.values[1].id, "2");
        assert_eq!(field.values[1].name, "Value 2");
        assert!(!field.values[1].selected);
        assert_eq!(field.values[1].description, "");
        assert!(!field.values[1].other_text);
        assert_eq!(field.values[1].other_text_description, "");
    }

    #[test]
    fn test_radio_buttons_field_deserializes() {
        let json = r#"{
            "$type": "Core.Models.Fields.Selection.SelectionField, Core",
            "values": [
              {
                "$type": "Core.Models.Fields.ValuesList.ValuesListValues, Core",
                "id": "1",
                "name": "Value 1",
                "selected": false,
                "description": "",
                "otherText": false,
                "otherTextDescription": "",
                "otherTextDefaultValue": "",
                "otherTextRequired": ""
              },
              {
                "$type": "Core.Models.Fields.ValuesList.ValuesListValues, Core",
                "id": "2",
                "name": "Value 2",
                "selected": false,
                "description": "",
                "otherText": false,
                "otherTextDescription": "",
                "otherTextDefaultValue": "",
                "otherTextRequired": ""
              }
            ],
            "controlType": "radio",
            "selectionType": "single",
            "id": "asfyh",
            "name": "radio buttons",
            "key": "radio-buttons",
            "fieldType": "valuesList",
            "required": false,
            "readOnly": false,
            "supportsMultipleOutputMappings": false
          }"#;

        let field: RadioButtonsField = serde_json::from_str(json).unwrap();

        assert_eq!(field.id, "asfyh");
        assert_eq!(field.name, "radio buttons");
        assert_eq!(field.key, "radio-buttons");
        assert_eq!(field.field_type, ValuesListConstant::ValuesList);
        assert_eq!(field.values.len(), 2);
        assert_eq!(field.values[0].id, "1");
        assert_eq!(field.values[0].name, "Value 1");
        assert!(!field.values[0].selected);
        assert_eq!(field.values[0].description, "");
        assert!(!field.values[0].other_text);
        assert_eq!(field.values[0].other_text_description, "");
        assert_eq!(field.values[0].other_text_default_value, "");
        assert_eq!(field.values[0].other_text_required, "");
        assert_eq!(field.values[1].id, "2");
        assert_eq!(field.values[1].name, "Value 2");
        assert!(!field.values[1].selected);
        assert_eq!(field.values[1].description, "");
        assert!(!field.values[1].other_text);
        assert_eq!(field.values[1].other_text_description, "");
    }

    #[test]
    fn test_checkboxes_field_deserializes() {
        let json = r#"{
          "$type": "Core.Models.Fields.ValuesListField, Core",
          "values": [
            {
              "$type": "Core.Models.Fields.ValuesList.ValuesListValues, Core",
              "id": "642afdc6c0476e1ea9a0138e",
              "name": "test2",
              "selected": false,
              "description": "<p>test</p>",
              "otherText": false,
              "otherTextDescription": "",
              "otherTextDefaultValue": "",
              "otherTextRequired": "False"
            },
            {
              "$type": "Core.Models.Fields.ValuesList.ValuesListValues, Core",
              "id": "642afdbf9e2b6a5fca95f60b",
              "name": "test",
              "selected": true,
              "description": "",
              "otherText": false,
              "otherTextDescription": "",
              "otherTextDefaultValue": "",
              "otherTextRequired": "False"
            }
          ],
          "controlType": "checkbox",
          "selectionType": "multi",
          "id": "a5jd9",
          "name": "Checkboxes",
          "key": "checkboxes",
          "fieldType": "valuesList",
          "required": false,
          "readOnly": false,
          "supportsMultipleOutputMappings": false
        }"#;

        let field: CheckboxesField = serde_json::from_str(json).unwrap();

        assert_eq!(field.id, "a5jd9");
        assert_eq!(field.name, "Checkboxes");
        assert_eq!(field.key, "checkboxes");
        assert_eq!(field.field_type, ValuesListConstant::ValuesList);
        assert_eq!(field.values.len(), 2);
        assert_eq!(field.values[0].id, "642afdc6c0476e1ea9a0138e");
        assert_eq!(field.values[0].name, "test2");
        assert!(!field.values[0].selected);
        assert_eq!(field.values[0].description, "<p>test</p>");
        assert!(!field.values[0].other_text);
        assert_eq!(field.values[0].other_text_description, "");
        assert_eq!(field.values[0].other_text_default_value, "");
        assert_eq!(field.values[0].other_text_required, "False");
        assert_eq!(field.values[1].id, "642afdbf9e2b6a5fca95f60b");
        assert_eq!(field.values[1].name, "test");
        assert!(field.values[1].selected);
        assert_eq!(field.values[1].description, "");
        assert!(!field.values[1].other_text);
        assert_eq!(field.values[1].other_text_description, "");
        assert_eq!(field.values[1].other_text_default_value, "");
        assert_eq!(field.values[1].other_text_required, "False");
    }
}
