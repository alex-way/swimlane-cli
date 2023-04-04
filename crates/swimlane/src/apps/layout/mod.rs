use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum LayoutType {
    Field,
    Section,
    HtmlObject,
    Tab,
    Integration,
    Tabs,
    Widget,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaseLayout {
    #[serde(rename = "$type")]
    pub _type: String,
    pub layout_type: LayoutType,
    pub id: String,
    pub parent_id: Option<String>,
    pub row: u64,
    pub col: u64,
    #[serde(rename = "sizex")]
    pub size_x: f32,
    #[serde(rename = "sizey")]
    pub size_y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct FieldLayout {
    #[serde(flatten)]
    pub base: BaseLayout,
    pub field_id: String,
    pub help_text_type: String,
    pub help_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct SectionLayout {
    #[serde(flatten)]
    pub base: BaseLayout,
    pub children: Vec<Layout>,
    pub collapsed: bool,
    pub container_hidden: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    #[serde(flatten)]
    pub base: BaseLayout,
    pub children: Vec<Layout>,
    pub name: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct TabsLayout {
    #[serde(flatten)]
    pub base: BaseLayout,
    pub tabs: Vec<Tab>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct HtmlObjectLayout {
    #[serde(flatten)]
    pub base: BaseLayout,
    pub html: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationLayout {
    #[serde(flatten)]
    pub base: BaseLayout,
    pub task_id: String,
    pub help_text_type: Option<String>,
    pub help_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct WidgetLayout {
    #[serde(flatten)]
    pub base: BaseLayout,
    pub name: String,
    pub code: String,
    pub help_text_type: Option<String>,
    pub help_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Layout {
    Field(FieldLayout),
    Section(SectionLayout),
    HtmlObject(HtmlObjectLayout),
    Integration(IntegrationLayout),
    Tabs(TabsLayout),
    Widget(WidgetLayout),
}
