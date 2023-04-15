use serde::{Deserialize, Serialize};

serde_enum!(TextConstant, { Text });
serde_enum!(NumericConstant, { Numeric });
serde_enum!(ValuesListConstant, { ValuesList });
serde_enum!(DateConstant, { Date });
serde_enum!(TimeConstant, { Time });
serde_enum!(DateTimeConstant, { DateTime });
serde_enum!(TimespanConstant, { Timespan });
serde_enum!(UserGroupConstant, { UserGroup });
serde_enum!(AttachmentConstant, { Attachment });
serde_enum!(TrackingConstant, { Tracking });
serde_enum!(ReferenceConstant, { Reference });
serde_enum!(CommentsConstant, { Comments });
serde_enum!(HistoryConstant, { History });
serde_enum!(ListConstant, { List });

serde_enum!(UserConstant, { User });
serde_enum!(SingleConstant, { Single });
serde_enum!(MultiConstant, { Multi });
serde_enum!(MultipleConstant, { Multiple });

serde_enum!(MultilineConstant, { Multiline });
serde_enum!(EmailConstant, { Email });
serde_enum!(TelephoneConstant, { Telephone });
serde_enum!(UrlConstant, { Url });
serde_enum!(IpConstant, { Ip });
serde_enum!(RichConstant, { Rich });
serde_enum!(JsonConstant, { Json });

serde_enum!(SelectConstant, { Select });
serde_enum!(RadioConstant, { Radio });
serde_enum!(CheckboxConstant, { Checkbox });

serde_enum!(FirstCreatedConstant, { FirstCreated });
serde_enum!(LastUpdatedConstant, { LastUpdated });
