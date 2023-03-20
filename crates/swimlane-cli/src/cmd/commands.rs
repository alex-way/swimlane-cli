use crate::error::SwimlaneCliError;
use crate::Migrate;
use colored::Colorize;
use std::path::PathBuf;
use swimlane::SwimlaneClient;
use swimlane_migrator::{MigrationPlan, SwimlaneMigrator};

pub async fn download_python_tasks(
    swimlane_client: &SwimlaneClient,
    path: &PathBuf,
) -> Result<(), SwimlaneCliError> {
    swimlane_client.download_python_tasks(&path).await?;
    Ok(())
}

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

    match migration_type {
        Migrate::Users {} => match dry_run {
            true => {
                println!("Dry run enabled, no changes will be made");
                let users = migrator.get_users_to_migrate().await?;

                if users.is_empty() {
                    println!(
                        "{}",
                        "No changed detected. Everything is up to date.".green()
                    );
                }

                // todo: order by type of change (create, update, delete)
                // todo: Extract into trait on Vec<MigrationPlan>
                for user in users {
                    match user {
                        MigrationPlan::Create { source_resource } => {
                            println!(
                                "{}",
                                format!("User: {} will be created", source_resource.name).green()
                            );
                        }
                        MigrationPlan::Update {
                            source_resource,
                            destination_resource: _,
                        } => {
                            println!(
                                "{}",
                                format!("User: {} will be updated", source_resource.name).yellow()
                            )
                        }
                        MigrationPlan::Delete {
                            destination_resource,
                        } => {
                            println!(
                                "{}",
                                format!("User: {} will be deleted", destination_resource.name)
                                    .red()
                            )
                        }
                    }
                }
            }
            false => migrator.migrate_users().await?,
        },
        Migrate::User { user_id: _ } => {
            todo!();
        }
        Migrate::Groups {} => match dry_run {
            true => {
                println!("Dry run enabled, no changes will be made");
                let groups = migrator.get_groups_to_migrate().await?;

                if groups.is_empty() {
                    println!(
                        "{}",
                        "No changed detected. Everything is up to date.".green()
                    );
                }

                // todo: order by type of change (create, update, delete)
                // todo: Extract into trait on Vec<MigrationPlan>
                for group in groups {
                    match group {
                        MigrationPlan::Create { source_resource } => {
                            println!(
                                "{}",
                                format!("Group: {} will be created", source_resource.name).green()
                            );
                        }
                        MigrationPlan::Update {
                            source_resource,
                            destination_resource: _,
                        } => {
                            println!(
                                "{}",
                                format!("Group: {} will be updated", source_resource.name).yellow()
                            )
                        }
                        MigrationPlan::Delete {
                            destination_resource,
                        } => {
                            println!(
                                "{}",
                                format!("Group: {} will be deleted", destination_resource.name)
                                    .red()
                            )
                        }
                    }
                }
            }
            false => migrator.migrate_groups().await?,
        },
        Migrate::Group { group_id: _ } => {
            todo!();
        }
        Migrate::Roles {} => match dry_run {
            true => {
                println!("Dry run enabled, no changes will be made");
                let roles = migrator.get_roles_to_migrate().await?;

                if roles.is_empty() {
                    println!(
                        "{}",
                        "No changed detected. Everything is up to date.".green()
                    );
                }

                // todo: order by type of change (create, update, delete)
                // todo: Extract into trait on Vec<MigrationPlan>
                for role in roles {
                    match role {
                        MigrationPlan::Create { source_resource } => {
                            println!(
                                "{}",
                                format!("Role: {} will be created", source_resource.name).green()
                            );
                        }
                        MigrationPlan::Update {
                            source_resource,
                            destination_resource: _,
                        } => {
                            println!(
                                "{}",
                                format!("Role: {} will be updated", source_resource.name).yellow()
                            );
                        }
                        MigrationPlan::Delete {
                            destination_resource,
                        } => {
                            println!(
                                "{}",
                                format!("Role: {} will be deleted", destination_resource.name)
                                    .red()
                            );
                        }
                    }
                }
            }
            false => migrator.migrate_groups().await?,
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
