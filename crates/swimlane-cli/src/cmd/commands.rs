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
    source: SwimlaneClient,
    target: SwimlaneClient,
    migration_type: Migrate,
    dry_run: bool,
) -> Result<(), SwimlaneCliError> {
    println!(
        "Migrating from {} to {}",
        &source.base_url, &target.base_url
    );

    let migrator = SwimlaneMigrator::new(source, target, dry_run)?;

    if dry_run {
        println!("Dry run enabled, no changes will be made");
    }

    match migration_type {
        Migrate::Users => match dry_run {
            true => {
                let users = migrator.get_users_to_migrate().await?;
                dry_run_resource_migrate(users);
            }
            false => migrator.migrate_users().await?,
        },
        Migrate::User { user_id: _ } => {
            todo!();
        }
        Migrate::Groups => match dry_run {
            true => {
                let groups = migrator.get_groups_to_migrate().await?;
                dry_run_resource_migrate(groups);
            }
            false => migrator.migrate_groups().await?,
        },
        Migrate::Group { group_id: _ } => {
            todo!();
        }
        Migrate::Roles => match dry_run {
            true => {
                let roles = migrator.get_roles_to_migrate().await?;
                dry_run_resource_migrate(roles);
            }
            false => migrator.migrate_roles().await?,
        },
        Migrate::Role { role_id: _ } => {
            todo!();
        }
        Migrate::Apps => match dry_run {
            true => {
                let apps = migrator.get_apps_to_migrate().await?;
                dry_run_resource_migrate(apps);
            }
            false => {
                // todo: prevent migrating apps where the acronym will change
                migrator.migrate_apps().await?
            }
        },
        Migrate::App {
            application_name: _,
        } => {
            todo!();
        }
        Migrate::All => {
            todo!();
        }
    }
    Ok(())
}
