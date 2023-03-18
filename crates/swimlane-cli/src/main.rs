use clap::{arg, Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use swimlane::SwimlaneClient;
use swimlane_migrator::{MigrationPlan, SwimlaneMigrator};
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(name = "swimlane-cli")]
#[command(about = "A simple CLI for interacting with Swimlane", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, env = "SWIMLANE_CLI__SOURCE_SWIMLANE_URL")]
    source_swimlane_url: String,

    #[arg(long, env = "SWIMLANE_CLI__SOURCE_SWIMLANE_PAT")]
    source_swimlane_pat: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Downloads all custom python tasks to a specified path
    #[command(arg_required_else_help = true)]
    DownloadPythonTasks {
        /// Where the tasks will be downloaded to
        path: PathBuf,
    },
    /// Subcommands for interacting with python packages
    Pip {
        #[clap(subcommand)]
        subcommand: Pip,
    },
    /// Migrates data from the source Swimlane server to the destination Swimlane server
    Migrate {
        #[clap(subcommand)]
        migration_type: Migrate,
        #[arg(long, env = "SWIMLANE_CLI__DESTINATION_SWIMLANE_URL")]
        destination_swimlane_url: String,
        #[arg(long, env = "SWIMLANE_CLI__DESTINATION_SWIMLANE_PAT")]
        destination_swimlane_pat: String,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        auto_approve: bool,
    },
}

#[derive(Debug, Subcommand)]
enum Migrate {
    /// Migrates all users from the source Swimlane server to the destination Swimlane server
    Users,
    /// Migrates the specified user from the source Swimlane server to the destination Swimlane server
    #[command(arg_required_else_help = true)]
    User { user_id: String },
    /// Migrates all groups from the source Swimlane server to the destination Swimlane server
    Groups,
    /// Migrates the specified group from the source Swimlane server to the destination Swimlane server
    #[command(arg_required_else_help = true)]
    Group { group_id: String },
    /// Migrates all roles from the source Swimlane server to the destination Swimlane server
    Roles,
    /// Migrates the specified role from the source Swimlane server to the destination Swimlane server
    #[command(arg_required_else_help = true)]
    Role { role_id: String },
    /// Migrates all applications from the source Swimlane server to the destination Swimlane server
    Apps,
    /// Migrates the specified application from the source Swimlane server to the destination Swimlane server
    #[command(arg_required_else_help = true)]
    App { application_name: String },

    /// Migrates all possible content from the source Swimlane server to the destination Swimlane server
    All,
}

#[derive(Debug, Subcommand)]
enum Pip {
    /// Installs a specified package or requirements.txt file
    #[command(arg_required_else_help = true)]
    Install {
        #[arg(required = false, short = 'r', required_unless_present_any = ["package"])]
        requirements_file: Option<PathBuf>,
        #[arg(required = false, required_unless_present_any = ["requirements_file"])]
        package: Option<String>,
    },
    /// Migrates all groups from the source Swimlane server to the destination Swimlane server
    #[command(arg_required_else_help = true)]
    Remove { package_name: String },
    /// Migrates all roles from the source Swimlane server to the destination Swimlane server
    Freeze,
}

#[derive(Error, Debug)]
pub enum SwimlaneCliError {
    #[error("Swimlane error")]
    SwimlaneError(#[from] swimlane::error::SwimlaneClientError),
    #[error("Error occurred whilst trying to create a new Swimlane migrator instance")]
    SwimlaneMigratorCreationError(#[from] swimlane_migrator::SwimlaneMigratorNewError),
    #[error("Swimlane migrator error")]
    SwimlaneMigratorError(#[from] swimlane_migrator::SwimlaneMigratorError),
    #[error("No package or requirements file specified")]
    NoPackageOrRequirementsFileSpecified,
    #[error("Package {0} does not exist")]
    PackageDoesNotExist(String),
    #[error("Generic error")]
    GenericError(#[source] Box<dyn std::error::Error>),
}

#[tokio::main]
async fn main() -> Result<(), SwimlaneCliError> {
    let args = Cli::parse();

    let swimlane_client = SwimlaneClient::new(args.source_swimlane_url, args.source_swimlane_pat);

    match args.command {
        Commands::DownloadPythonTasks { path } => {
            download_python_tasks(&swimlane_client, &path).await?
        }
        Commands::Pip { subcommand } => match subcommand {
            Pip::Install {
                requirements_file,
                package,
            } => {
                if let Some(requirements_file) = requirements_file {
                    swimlane_client
                        .upload_python_requirements(&requirements_file)
                        .await?;
                } else if let Some(package) = package {
                    let package_version = parse_package_version(&package);

                    swimlane_client
                        .install_pip_package(&package, package_version)
                        .await?;
                } else {
                    return Err(SwimlaneCliError::NoPackageOrRequirementsFileSpecified);
                }
            }
            Pip::Remove { package_name } => {
                remove_python_package(&swimlane_client, &package_name).await?
            }
            Pip::Freeze {} => freeze_python_packages(&swimlane_client).await?,
        },
        Commands::Migrate {
            migration_type,
            destination_swimlane_url,
            destination_swimlane_pat,
            dry_run,
            auto_approve: _,
        } => {
            handle_migrate(
                swimlane_client,
                migration_type,
                destination_swimlane_url,
                destination_swimlane_pat,
                dry_run,
            )
            .await?
        }
    }
    Ok(())
}

async fn download_python_tasks(
    swimlane_client: &SwimlaneClient,
    path: &PathBuf,
) -> Result<(), SwimlaneCliError> {
    swimlane_client.download_python_tasks(&path).await?;
    Ok(())
}

async fn remove_python_package(
    swimlane_client: &SwimlaneClient,
    package_name: &str,
) -> Result<(), SwimlaneCliError> {
    swimlane_client.uninstall_pip_package(package_name).await?;
    Ok(())
}

async fn freeze_python_packages(swimlane_client: &SwimlaneClient) -> Result<(), SwimlaneCliError> {
    let packages = swimlane_client.get_installed_pip_packages().await?;
    for package in packages {
        let package_version = package.version.unwrap_or("latest".to_string());
        println!("{}=={}", package.name, package_version);
    }
    Ok(())
}

async fn handle_migrate(
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
        Migrate::Users {} => {
            migrator.migrate_users().await?;
        }
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
                for group in groups {
                    match group {
                        MigrationPlan::Create { source_resource } => {
                            println!("Group: {} will be created", source_resource.name.green());
                        }
                        MigrationPlan::Update {
                            source_resource,
                            destination_resource: _,
                        } => {
                            println!("Group: {} will be updated", source_resource.name.yellow());
                        }
                        MigrationPlan::Delete {
                            destination_resource,
                        } => {
                            println!("Group: {} will be deleted", destination_resource.name.red());
                        }
                    }
                }
            }
            false => migrator.migrate_groups().await?,
        },
        Migrate::Group { group_id: _ } => {
            todo!();
        }
        Migrate::Roles {} => {
            migrator.migrate_roles().await;
        }
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

fn parse_package_version(package_version: &str) -> Option<&str> {
    let package_version = package_version.trim();
    match package_version.is_empty() {
        true => None,
        false => {
            let split = package_version.split("==").collect::<Vec<&str>>();
            match split.len() {
                2 => Some(split[1]),
                _ => None,
            }
        }
    }
}
