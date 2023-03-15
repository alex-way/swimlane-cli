use swimlane::{Group, User};

pub trait LooksLike {
    /// Whether the main fields of the two objects are identical
    /// For example,
    ///
    /// ```
    /// let a = User {
    ///    id: "2".to_string()
    ///    user_name: "user1".to_string()
    /// }
    ///
    /// let b = User {
    ///   id: "1".to_string()
    ///   user_name: "user1".to_string()
    /// }
    ///
    /// a.looks_like(&b) // true
    /// ```
    ///
    /// Notice that the id is different, but the user_name is the same.
    /// This is because the user_name is the main field that we care about.
    /// The id is just a unique identifier.
    fn looks_like(&self, other: &Self) -> bool;

    // todo: implement a differences function that returns a list of differences and create default implementation for
    // looks_like that checks if the differences list is empty
}

impl LooksLike for Group {
    fn looks_like(&self, other: &Self) -> bool {
        // todo: compare group membership, role membership, users
        self.description == other.description && self.disabled == other.disabled
    }
}

impl LooksLike for User {
    fn looks_like(&self, other: &Self) -> bool {
        // todo: compare group membership, role membership, primary group, phone, time zone, default dashboard, profile image, middle initial
        self.display_name == other.display_name
            && self.disabled == other.disabled
            && self.email == other.email
            && self.first_name == other.first_name
            && self.last_name == other.last_name
    }
}
