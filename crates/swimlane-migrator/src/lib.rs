// use swimlane::SwimlaneClient;

// pub fn migrate_users(from: &SwimlaneClient, to: &SwimlaneClient) {
//     let users = from.get_users().unwrap();
//     for user in users {
//         to.create_user(&user).unwrap();
//     }
// }

// pub fn migrate_groups(from: &SwimlaneClient, to: &SwimlaneClient) {
//     let groups = from.get_groups().unwrap();
//     for group in groups {
//         to.create_group(&group).unwrap();
//     }
// }

// pub fn migrate_roles(from: &SwimlaneClient, to: &SwimlaneClient) {
//     let roles = from.get_roles().unwrap();
//     for role in roles {
//         to.create_role(&role).unwrap();
//     }
// }

// pub fn migrate_apps(from: &SwimlaneClient, to: &SwimlaneClient) {
//     let apps = from.get_apps().unwrap();
//     for app in apps {
//         to.create_app(&app).unwrap();
//     }
// }
