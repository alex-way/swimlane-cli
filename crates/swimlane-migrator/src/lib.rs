use swimlane::SwimlaneClient;

pub struct SwimlaneMigrator {
    pub from: SwimlaneClient,
    pub to: SwimlaneClient,
}

impl SwimlaneMigrator {
    pub fn new(from: SwimlaneClient, to: SwimlaneClient) -> Self {
        Self { from, to }
    }

    pub async fn migrate_users(&self) {
        todo!()
    }

    pub async fn migrate_groups(&self) {
        todo!()
    }

    pub async fn migrate_roles(&self) {
        todo!()
    }

    pub async fn migrate_apps(&self) {
        todo!()
    }
}
