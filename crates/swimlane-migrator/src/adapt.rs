use std::collections::HashMap;

use swimlane::groups::Group;

use crate::SwimlaneMigrator;
// use swimlane::users::User;

// pub trait Adapt {
//     /// Adapts a struct to match another struct
//     /// For example,
//     /// ```rust
//     /// use swimlane_migrator::adapt::Adapt;
//     ///
//     /// struct User {
//     ///  id: String,
//     /// user_name: String,
//     /// }
//     ///
//     /// impl Adapt for User {
//     /// fn adapt(&mut self, other: &Self) -> Self {
//     /// self.user_name = other.user_name.clone();
//     /// }
//     /// }
//     ///
//     /// let mut a = User {
//     /// id: "2".to_string(),
//     /// user_name: "user1".to_string(),
//     /// };
//     ///
//     /// let b = User {
//     /// id: "1".to_string(),
//     /// user_name: "user2".to_string(),
//     /// };
//     ///
//     /// a.adapt(&b);
//     ///
//     /// assert_eq!(a.user_name, "user2".to_string())
//     /// ```
//     fn adapt(&mut self, other: &Self) -> Self;
// }

// impl Adapt for Group {
//     fn adapt(&mut self, other: &Self) -> Self {
//         if self.looks_like(other) {
//             return self.clone();
//         }
//     }
// }

impl SwimlaneMigrator {
    pub async fn adapt_group(
        &self,
        group: &mut Group,
        group_id_hashmap: &HashMap<String, String>,
        user_id_hashmap: &HashMap<String, String>,
        role_id_hashmap: &HashMap<String, String>,
    ) {
        for user in &mut group.users {
            if let Some(new_id) = user_id_hashmap.get(&user.id) {
                user.id = new_id.clone();
            }
        }

        for role in &mut group.roles {
            if let Some(new_id) = role_id_hashmap.get(&role.id) {
                role.id = new_id.clone();
            }
        }

        for child_group in &mut group.groups {
            if let Some(new_id) = group_id_hashmap.get(&child_group.id) {
                child_group.id = new_id.clone();
            }
        }
    }
}
