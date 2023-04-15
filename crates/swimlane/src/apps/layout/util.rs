macro_rules! define_layout {
    ($struct_identifier:ident, $layout_type:ident $(, {$($(#[$meta:meta])* $vis:vis $field:ident: $ty:ty),* $(,)? })?) => {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "camelCase")]
        pub struct $struct_identifier {
            #[serde(rename = "$type")]
            pub _type: String,
            pub layout_type: $layout_type,
            pub id: String,
            pub parent_id: Option<String>,
            pub row: u64,
            pub col: u64,
            #[serde(rename = "sizex")]
            pub size_x: f32,
            #[serde(rename = "sizey")]
            pub size_y: f32,
            $($($vis $field: $ty),*)?
        }
    };
}
