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
