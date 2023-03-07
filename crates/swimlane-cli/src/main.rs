use clap::{arg, Parser, Subcommand};
use std::path::PathBuf;
use swimlane::SwimlaneClient;

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
    Role { group_id: String },
    /// Migrates all applications from the source Swimlane server to the destination Swimlane server
    Applications,
    /// Migrates the specified application from the source Swimlane server to the destination Swimlane server
    #[command(arg_required_else_help = true)]
    Application { application_name: String },

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
    Remove,
    /// Migrates all roles from the source Swimlane server to the destination Swimlane server
    Freeze,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let swimlane_client = SwimlaneClient::new(args.source_swimlane_url, args.source_swimlane_pat);

    swimlane_client.health_ping().await?;

    match args.command {
        Commands::DownloadPythonTasks { path } => {
            swimlane_client.download_python_tasks(&path).await?;
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
                    return Err("No package or requirements file specified".into());
                }
            }
            Pip::Remove {} => {
                return Err("Remove is not supported".into());
            }
            Pip::Freeze {} => {
                let packages = swimlane_client.get_installed_pip_packages().await?;
                for package in packages {
                    let package_version = package.version.unwrap_or("latest".to_string());
                    println!("{}=={}", package.name, package_version);
                }
            }
        },
        Commands::Migrate {
            migration_type,
            destination_swimlane_url,
            destination_swimlane_pat,
        } => match migration_type {
            Migrate::Users {} => {
                println!("{}, {}", destination_swimlane_url, destination_swimlane_pat);
            }
            Migrate::User { user_id: _ } => {
                return Err("User migration is not supported".into());
            }
            Migrate::Groups {} => {
                return Err("Group migration is not supported".into());
            }
            Migrate::Group { group_id: _ } => {
                return Err("Group migration is not supported".into());
            }
            Migrate::Roles {} => {
                return Err("Role migration is not supported".into());
            }
            Migrate::Role { group_id: _ } => {
                return Err("Role migration is not supported".into());
            }
            Migrate::Applications {} => {
                return Err("Application migration is not supported".into());
            }
            Migrate::Application {
                application_name: _,
            } => {
                return Err("Application migration is not supported".into());
            }
            Migrate::All {} => {
                return Err("All migration is not supported".into());
            }
        },
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
