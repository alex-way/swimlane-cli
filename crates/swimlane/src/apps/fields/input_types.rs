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
