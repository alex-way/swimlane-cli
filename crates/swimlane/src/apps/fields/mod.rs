#[macro_use]
mod util;
pub mod constants;
pub mod datetime;
pub mod input_types;
pub mod numeric;
pub mod reference;
pub mod selection;
pub mod text;
pub mod users_groups;

use serde::{Deserialize, Serialize};

use self::constants::{AttachmentConstant, CommentsConstant, HistoryConstant, TrackingConstant};

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

impl Field {
    pub fn name(&self) -> String {
        match self {
            Field::SingleLineText(field) => field.name.clone(),
            Field::MultiLineText(field) => field.name.clone(),
            Field::Email(field) => field.name.clone(),
            Field::Telephone(field) => field.name.clone(),
            Field::Url(field) => field.name.clone(),
            Field::IpAddress(field) => field.name.clone(),
            Field::RichText(field) => field.name.clone(),
            Field::Json(field) => field.name.clone(),
            Field::TextList(field) => field.name.clone(),
            Field::Numeric(field) => field.name.clone(),
            Field::NumericList(field) => field.name.clone(),
            Field::DateTime(field) => field.name.clone(),
            Field::Date(field) => field.name.clone(),
            Field::Time(field) => field.name.clone(),
            Field::TimeSpan(field) => field.name.clone(),
            Field::FirstCreated(field) => field.name.clone(),
            Field::LastUpdated(field) => field.name.clone(),
            Field::SingleSelect(field) => field.name.clone(),
            Field::MultiSelect(field) => field.name.clone(),
            Field::RadioButtons(field) => field.name.clone(),
            Field::Checkboxes(field) => field.name.clone(),
            Field::SingleUserGroup(field) => field.name.clone(),
            Field::MultiUserGroup(field) => field.name.clone(),
            Field::CreatedBy(field) => field.name.clone(),
            Field::LastUpdatedBy(field) => field.name.clone(),
            Field::Correlation(field) => field.name.clone(),
            Field::SingleReference(field) => field.name.clone(),
            Field::MultiReference(field) => field.name.clone(),
            Field::GridReference(field) => field.name.clone(),
            Field::TrackingId(field) => field.name.clone(),
            Field::Attachment(field) => field.name.clone(),
            Field::Comments(field) => field.name.clone(),
            Field::History(field) => field.name.clone(),
        }
    }
}

define_field!(AttachmentsField, AttachmentConstant, {
    pub max_size: u64,
    pub time_to_live: Option<u64>,
    /// Comma separated list of file extensions. e.g. "jpg,png,gif"
    pub supported_file_types: Option<Vec<String>>,
});

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
    pub field_type: TrackingConstant,
    /// Always true
    pub read_only: bool,
    /// Always false
    pub supports_multiple_output_mappings: bool,
}

define_field!(CommentsField, CommentsConstant);
define_field!(HistoryField, HistoryConstant);
