#[macro_use]
mod util;
pub mod datetime;
pub mod numeric;
pub mod reference;
pub mod selection;
pub mod text;
pub mod users_groups;

use serde::{Deserialize, Serialize};

serde_enum!(FieldType, {
    None,
    Text,
    Numeric,
    ValuesList,
    Date,
    UserGroup,
    Attachment,
    Tracking,
    Reference,
    Comments,
    History,
    List,
});

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Field {
    SingleLineText(text::SingleLineTextField),
    MultiLineText(text::MultiLineTextField),
    Email(text::EmailField),
    Telephone(text::TelephoneField),
    Url(text::UrlField),
    IpAddress(text::IpAddressField),
    RichText(text::RichTextField),
    Json(text::JsonField),
    TextList(text::TextListField),
    Numeric(numeric::NumericField),
    NumericList(numeric::NumericListField),
    DateTime(datetime::DateTimeField),
    Date(datetime::DateField),
    Time(datetime::TimeField),
    TimeSpan(datetime::TimeSpanField),
    FirstCreated(datetime::FirstCreatedField),
    LastUpdated(datetime::LastUpdatedField),
    SingleSelect(selection::SingleSelectField),
    MultiSelect(selection::MultiSelectField),
    RadioButtons(selection::RadioButtonsField),
    Checkboxes(selection::CheckboxesField),
    SingleUserGroup(users_groups::SingleUserGroupField),
    MultiUserGroup(users_groups::MultiUserGroupField),
    CreatedBy(users_groups::CreatedByField),
    LastUpdatedBy(users_groups::LastUpdatedByField),
    Correlation(reference::CorrelationField),
    SingleReference(reference::SingleReferenceField),
    MultiReference(reference::MultiReferenceField),
    GridReference(reference::GridReferenceField),
    TrackingId(TrackingIdField),
    Attachment(AttachmentsField),
    Comments(CommentsField),
    History(HistoryField),
}

/// Contains all the base fields for any field in Swimlane
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaseField {
    #[serde(rename = "$type")]
    pub _type: String,
    pub id: String,
    pub name: String,
    pub key: String,
    pub supports_multiple_output_mappings: bool,
    pub required: bool,
    pub read_only: bool,
}

serde_enum!(AttachmentFieldType, { Attachment });

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AttachmentsField {
    #[serde(flatten)]
    pub base: BaseField,
    pub field_type: AttachmentFieldType,
    pub max_size: u64,
    pub time_to_live: Option<u64>,
    /// Comma separated list of file extensions. e.g. "jpg,png,gif"
    pub supported_file_types: Option<Vec<String>>,
}

serde_enum!(TrackingIdFieldType, { Tracking });

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TrackingIdField {
    #[serde(rename = "$type")]
    pub _type: String,
    /// The prefix of the application, suffixed with a hyphen. e.g. "APP-"
    pub prefix: String,
    pub id: String,
    /// Always "Tracking Id"
    pub name: String,
    /// Always "tracking-id"
    pub key: String,
    /// Always "tracking"
    pub field_type: TrackingIdFieldType,
    /// Always true
    pub read_only: bool,
    /// Always false
    pub supports_multiple_output_mappings: bool,
}

serde_enum!(CommentsFieldType, { Comments });

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CommentsField {
    #[serde(flatten)]
    pub base: BaseField,
    pub field_type: CommentsFieldType,
}

serde_enum!(HistoryFieldType, { History });

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HistoryField {
    #[serde(flatten)]
    pub base: BaseField,
    pub field_type: HistoryFieldType,
}
