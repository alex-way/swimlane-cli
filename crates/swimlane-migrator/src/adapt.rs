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
