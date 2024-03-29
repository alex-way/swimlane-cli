use std::fmt::{Display, Formatter};

use swimlane::BaseEntity;

#[derive(Debug, Clone, PartialEq)]
pub enum Difference {
    UpdatingField {
        field: String,
        current_value: String,
        new_value: String,
    },
    AddingItem {
        field: String,
        item: String,
    },
    RemovingItem {
        field: String,
        item: String,
    },
    /// Used for when the field is a complex type. For example the application type,
    /// where it wouldn't be possible to display the difference coherently.
    UpdatingComplexField {
        field: String,
    },
}

macro_rules! push_difference {
    ($differences:expr, $field:literal , $current_value:expr, $new_value:expr) => {
        if $current_value != $new_value {
            $differences.push(Difference::UpdatingField {
                field: $field.to_string(),
                current_value: $current_value.to_string(),
                new_value: $new_value.to_string(),
            });
        }
    };
    ($differences:expr, $field:literal, $current_value:expr, $new_value:expr, optional: true) => {
        if $current_value != $new_value {
            $differences.push(Difference::UpdatingField {
                field: $field.to_string(),
                current_value: $current_value
                    .clone()
                    .map_or("".to_string(), |v| v.to_string()),
                new_value: $new_value.clone().map_or("".to_string(), |v| v.to_string()),
            });
        }
    };
    ($differences:expr, $field:literal, $source_vec:expr, $target_vec:expr, vec: true) => {
        $differences.extend($source_vec.iter().filter_map(|item| {
            let item_exists = $target_vec
                .iter()
                .find(|other_item| item.is_same_resource(other_item));
            match item_exists {
                Some(_) => None,
                None => Some(Difference::AddingItem {
                    field: $field.to_string(),
                    item: item.name.clone(),
                }),
            }
        }));
        $differences.extend($target_vec.iter().filter_map(|item| {
            let item_exists = $source_vec
                .iter()
                .find(|other_item| item.is_same_resource(other_item));
            match item_exists {
                Some(_) => None,
                None => Some(Difference::RemovingItem {
                    field: $field.to_string(),
                    item: item.name.clone(),
                }),
            }
        }));
    };

    ($differences:expr, $field:literal, $source_vec:expr, $target_vec:expr, str_vec: true) => {
        $differences.extend($source_vec.iter().filter_map(|item| {
            let item_exists = $target_vec
                .iter()
                .find(|other_item| item.is_same_resource(other_item));
            match item_exists {
                Some(_) => None,
                None => Some(Difference::AddingItem {
                    field: $field.to_string(),
                    item: item.clone(),
                }),
            }
        }));
        $differences.extend($target_vec.iter().filter_map(|item| {
            let item_exists = $source_vec
                .iter()
                .find(|other_item| item.is_same_resource(other_item));
            match item_exists {
                Some(_) => None,
                None => Some(Difference::RemovingItem {
                    field: $field.to_string(),
                    item: item.clone(),
                }),
            }
        }));
    };
}

impl Display for Difference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Difference::UpdatingField {
                field,
                current_value,
                new_value,
            } => write!(f, "{}: '{}' -> '{}'", field, new_value, current_value),
            Difference::AddingItem { field, item } => {
                write!(f, "+{}: {}", field, item)
            }
            Difference::RemovingItem { field, item } => {
                write!(f, "-{}: {}", field, item)
            }
            Difference::UpdatingComplexField { field } => {
                write!(f, "{} will be updated", field)
            }
        }
    }
}

pub trait LooksLike {
    fn differences(&self, other: &Self) -> Vec<Difference>;

    /// Whether the main fields of the two objects are identical
    /// For example,
    ///
    /// ```rust
    /// use swimlane_migrator::equality::{LooksLike,Difference};
    ///
    /// struct User {
    ///   id: String,
    ///   user_name: String,
    /// }
    ///
    /// impl LooksLike for User {
    /// fn differences(&self, other: &Self) -> Vec<Difference> {
    ///  let mut differences = vec![];
    ///  differences
    /// }
    ///
    ///  fn looks_like(&self, other: &Self) -> bool {
    ///   self.user_name == other.user_name
    ///  }
    ///
    ///  fn is_same_resource(&self, other: &Self) -> bool {
    ///     self.id == other.id
    ///  }
    /// }
    ///
    ///
    /// let a = User {
    ///  id: "2".to_string(),
    ///  user_name: "user1".to_string(),
    /// };
    ///
    /// let b = User {
    ///  id: "1".to_string(),
    ///  user_name: "user1".to_string(),
    /// };
    ///
    /// assert_eq!(a.looks_like(&b), true)
    /// ```
    ///
    /// Notice that the id is different, but the user_name is the same.
    /// This is because the user_name is the main field that we care about.
    /// The id is just a unique identifier.
    fn looks_like(&self, other: &Self) -> bool {
        self.differences(other).is_empty()
    }

    /// Whether the two objects are the same resource
    /// For example,
    ///
    /// ```rust
    /// use swimlane_migrator::equality::{LooksLike,Difference};
    ///
    /// struct User {
    ///  id: String,
    /// user_name: String,
    /// }
    ///
    /// impl LooksLike for User {
    /// fn differences(&self, other: &Self) -> Vec<Difference> {
    ///  let mut differences = vec![];
    ///  differences
    /// }
    ///
    /// fn is_same_resource(&self, other: &Self) -> bool {
    ///     self.id == other.id
    /// }
    /// }
    ///
    /// let a = User {
    /// id: "2".to_string(),
    /// user_name: "user1".to_string(),
    /// };
    ///
    /// let b = User {
    /// id: "1".to_string(),
    /// user_name: "user1".to_string(),
    /// };
    ///
    /// assert_eq!(a.is_same_resource(&b), false)
    /// ```
    fn is_same_resource(&self, other: &Self) -> bool;
}

impl LooksLike for BaseEntity {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut differences = vec![];
        if self.name != other.name {
            differences.push(Difference::UpdatingField {
                field: "name".to_string(),
                current_value: self.name.clone(),
                new_value: other.name.clone(),
            });
        }
        if self.disabled != other.disabled {
            differences.push(Difference::UpdatingField {
                field: "disabled".to_string(),
                current_value: self.disabled.to_string(),
                new_value: other.disabled.to_string(),
            });
        }
        differences
    }

    fn looks_like(&self, other: &Self) -> bool {
        // disabled is omitted because it's uncontrollable from the BaseEntity. The property has to be set on the
        // specific entity type (User, Group, Role)
        self.name == other.name
    }

    fn is_same_resource(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
