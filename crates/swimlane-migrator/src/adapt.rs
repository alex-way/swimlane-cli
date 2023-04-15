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

use std::collections::HashMap;

use swimlane::{apps::Application, SwimlaneClient};

/// Allows for the "Normalization" of Swimlane Resources to convert reference IDs to common references, like name or username
pub struct SwimlaneResourceNormaliser {
    swimlane_client: SwimlaneClient,
}

impl SwimlaneResourceNormaliser {
    pub fn new(swimlane_client: SwimlaneClient) -> Self {
        Self { swimlane_client }
    }

    pub async fn get_workspace_hashmap(&self) -> HashMap<String, String> {
        let workspaces = self.swimlane_client.get_workspaces().await.unwrap();
        let mut workspace_hashmap = HashMap::new();
        for workspace in workspaces {
            workspace_hashmap.insert(workspace.id.clone(), workspace.name.clone());
            workspace_hashmap.insert(workspace.name, workspace.id);
        }
        workspace_hashmap
    }

    pub fn normalise_application(
        &self,
        app: &Application,
        workspace_hashmap: &HashMap<String, String>,
    ) -> Application {
        let mut app = app.clone();
        app.workspaces = app
            .workspaces
            .iter()
            .map(|workspace| workspace_hashmap.get(workspace).unwrap().to_string())
            .collect();

        // todo: handle permissions
        // todo: handle integration fields and fields in general

        app
    }
}
