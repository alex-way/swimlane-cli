// use swimlane::{Group, User};

// pub trait Adapt {
//     fn adapt(&self, other: &Self) -> Self;
// }

// impl Adapt for Group {
//     fn adapt(&self, other: &Self) -> Self {
//         // todo: compare group membership, role membership, users
//         self.description == other.description && self.disabled == other.disabled
//     }
// }

// impl Adapt for User {
//     fn adapt(&self, other: &Self) -> Self {
//         // todo: compare group membership, role membership, primary group, phone, time zone, default dashboard, profile image, middle initial
//         self.display_name == other.display_name
//             && self.disabled == other.disabled
//             && self.email == other.email
//             && self.first_name == other.first_name
//             && self.last_name == other.last_name
//     }
// }
