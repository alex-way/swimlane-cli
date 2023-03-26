use crate::error::SwimlaneCliError;
use crate::Migrate;
use swimlane::SwimlaneClient;
use swimlane_migrator::SwimlaneMigrator;

use super::migrate::dry_run_resource_migrate;

pub async fn remove_python_package(
    swimlane_client: &SwimlaneClient,
    package_name: &str,
) -> Result<(), SwimlaneCliError> {
    swimlane_client.uninstall_pip_package(package_name).await?;
    Ok(())
}

pub async fn freeze_python_packages(
    swimlane_client: &SwimlaneClient,
) -> Result<(), SwimlaneCliError> {
    let packages = swimlane_client.get_installed_pip_packages().await?;
    for package in packages {
        let package_version = package.version.unwrap_or("latest".to_string());
        println!("{}=={}", package.name, package_version);
    }
    Ok(())
}

pub async fn handle_migrate(
    swimlane_client: SwimlaneClient,
    migration_type: Migrate,
    destination_swimlane_url: String,
    destination_swimlane_pat: String,
    dry_run: bool,
) -> Result<(), SwimlaneCliError> {
    let destination_swimlane_client =
        SwimlaneClient::new(destination_swimlane_url, destination_swimlane_pat);

    let migrator = SwimlaneMigrator::new(swimlane_client, destination_swimlane_client, dry_run)?;

    if dry_run {
        println!("Dry run enabled, no changes will be made");
    }

    match migration_type {
        Migrate::Users {} => match dry_run {
            true => {
                let users = migrator.get_users_to_migrate().await?;
                dry_run_resource_migrate(users);
            }
            false => migrator.migrate_users().await?,
        },
        Migrate::User { user_id: _ } => {
            todo!();
        }
        Migrate::Groups {} => match dry_run {
            true => {
                let groups = migrator.get_groups_to_migrate().await?;
                dry_run_resource_migrate(groups);
            }
            false => migrator.migrate_groups().await?,
        },
        Migrate::Group { group_id: _ } => {
            todo!();
        }
        Migrate::Roles {} => match dry_run {
            true => {
                let roles = migrator.get_roles_to_migrate().await?;
                dry_run_resource_migrate(roles);
            }
            false => migrator.migrate_roles().await?,
        },
        Migrate::Role { role_id: _ } => {
            todo!();
        }
        Migrate::Apps {} => {
            migrator.migrate_apps().await;
        }
        Migrate::App {
            application_name: _,
        } => {
            todo!();
        }
        Migrate::All {} => {
            todo!();
        }
    }
    Ok(())
}
