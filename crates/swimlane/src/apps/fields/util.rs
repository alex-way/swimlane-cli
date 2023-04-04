/// A macro to create a serde enum with the following derived traits: Serialize, Deserialize, Debug, Clone, PartialEq
macro_rules! serde_enum {
    ($name:ident, { $($variant:ident),* $(,)? }) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum $name {
            $($variant),*
        }
    };
}

macro_rules! define_field {
    ($struct_identifier:ident, $field_type:ty $(, {$($(#[$meta:meta])* pub $field:ident: $ty:ty),* $(,)? })?) => {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "camelCase")]
        pub struct $struct_identifier {
            #[serde(rename = "$type")]
            pub _type: String,
            pub id: String,
            pub name: String,
            pub key: String,
            pub supports_multiple_output_mappings: bool,
            pub required: bool,
            pub read_only: bool,
            pub field_type: $field_type,
            $(pub $($field: $ty),*)?
        }
    };
}
