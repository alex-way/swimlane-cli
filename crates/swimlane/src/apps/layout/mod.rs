use serde::{Deserialize, Serialize};

use self::constants::{
    FieldConstant, HtmlObjectConstant, IntegrationConstant, SectionConstant, TabConstant,
    TabsConstant, WidgetConstant,
};

#[macro_use]
mod util;
pub mod constants;

serde_enum!(LayoutType, { Field, Section, HtmlObject, Tab, Integration, Tabs, Widget });

define_layout!(FieldLayout, FieldConstant, {
    pub field_id: String,
    pub help_text_type: String,
    pub help_text: Option<String>,
});

define_layout!(SectionLayout, SectionConstant, {
    pub name: String,
    pub children: Vec<Layout>,
    pub collapsed: bool,
    pub container_hidden: bool,
    pub help_text_type: Option<String>,
    pub help_text: Option<String>,
});

define_layout!(Tab, TabsConstant, {
    pub children: Vec<Layout>,
    pub name: String,
    pub active: bool,
});

define_layout!(TabsLayout, TabConstant, {
    pub tabs: Vec<Tab>,
});

define_layout!(HtmlObjectLayout, HtmlObjectConstant, {
    pub html: String,
    pub name: String,
});

define_layout!(IntegrationLayout, IntegrationConstant, {
    pub task_id: Option<String>,
    // No idea why but this is an i64, whereas the others are a string...
    pub help_text_type: i64,
    pub help_text: String,
    pub name: String,
});

define_layout!(WidgetLayout, WidgetConstant, {
    pub name: String,
    pub code: String,
    pub help_text_type: Option<String>,
    pub help_text: Option<String>,
});

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_deserializes() {
        let json = include_str!("./json/section.json");

        let layout: Layout = serde_json::from_str(json).unwrap();

        match layout {
            Layout::Section(section) => {
                assert_eq!(section.id, "aQihGKccUFCYcCA4a");
            }
            _ => panic!("Expected a section"),
        }
    }
}
