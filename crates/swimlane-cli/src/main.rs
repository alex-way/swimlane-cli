mod cmd;
pub mod error;
pub mod util;

use clap::{arg, Parser, Subcommand};
use cmd::commands::{freeze_python_packages, handle_migrate, remove_python_package};
use cmd::task::save_python_tasks;
use error::SwimlaneCliError;
use std::env;
use std::path::PathBuf;
use swimlane::SwimlaneClient;
use util::parse_package_version;

#[derive(Debug, Parser)]
#[command(name = "swimlane-cli")]
#[command(about = "A simple CLI for interacting with Swimlane", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long, env = "SWIMLANE_CLI__URL")]
    url: String,

    #[arg(long, env = "SWIMLANE_CLI__PAT")]
    pat: String,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Subcommands for interacting with tasks
    #[command(arg_required_else_help = true)]
    Task {
        #[clap(subcommand)]
        subcommand: Task,
    },
    /// Subcommands for interacting with python packages
    Pip {
        #[clap(subcommand)]
        subcommand: Pip,
    },
    /// Migrates data from the source Swimlane server to the target Swimlane server
    Migrate {
        #[clap(subcommand)]
        migration_type: Option<Migrate>,
        #[arg(long, env = "SWIMLANE_CLI__TARGET_URL")]
        target_url: String,
        #[arg(long, env = "SWIMLANE_CLI__TARGET_URL")]
        target_pat: String,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        auto_approve: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum Migrate {
    /// Migrates all users from the source Swimlane server to the target Swimlane server
    Users,
    /// Migrates the specified user from the source Swimlane server to the target Swimlane server
    #[command(arg_required_else_help = true)]
    User { user_id: String },
    /// Migrates all groups from the source Swimlane server to the target Swimlane server
    Groups,
    /// Migrates the specified group from the source Swimlane server to the target Swimlane server
    #[command(arg_required_else_help = true)]
    Group { group_id: String },
    /// Migrates all roles from the source Swimlane server to the target Swimlane server
    Roles,
    /// Migrates the specified role from the source Swimlane server to the target Swimlane server
    #[command(arg_required_else_help = true)]
    Role { role_id: String },
    /// Migrates all applications from the source Swimlane server to the target Swimlane server
    Apps,
    /// Migrates the specified application from the source Swimlane server to the target Swimlane server
    #[command(arg_required_else_help = true)]
    App { application_name: String },

    /// Migrates all possible content from the source Swimlane server to the target Swimlane server
    All,
}

#[derive(Debug, Subcommand)]
pub enum Pip {
    /// Installs a specified package or requirements.txt file
    #[command(arg_required_else_help = true)]
    Install {
        #[arg(required = false, short = 'r', required_unless_present_any = ["package"])]
        requirements_file: Option<PathBuf>,
        #[arg(required = false, required_unless_present_any = ["requirements_file"])]
        package: Option<String>,
    },
    /// Removes a specified package from the Swimlane server
    #[command(arg_required_else_help = true)]
    Remove { package_name: String },
    /// Lists all installed packages on the Swimlane server and produces output which is compatible with the requirements.txt format
    Freeze,
}

#[derive(Debug, Subcommand)]
pub enum Task {
    /// Downloads all custom python tasks to a specified path
    #[command(arg_required_else_help = true)]
    Save {
        /// Where the tasks will be downloaded to. Defaults to .
        #[arg(short, long, default_value=env::current_dir().unwrap().into_os_string())]
        path: PathBuf,
        /// Application to download tasks for
        #[arg(short, long)]
        app: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), SwimlaneCliError> {
    let args = Cli::parse();

    let swimlane_client = SwimlaneClient::new(args.url, args.pat);

    match args.command {
        Command::Task { subcommand } => match subcommand {
            Task::Save { path, app } => {
                println!("Downloading tasks to: '{}'", path.display());
                save_python_tasks(&swimlane_client, &path, &app).await?
            }
        },
        Command::Pip { subcommand } => match subcommand {
            Pip::Install {
                requirements_file,
                package,
            } => {
                if let Some(requirements_file) = requirements_file {
                    swimlane_client
                        .upload_python_requirements(&requirements_file)
                        .await?;
                } else if let Some(package) = package {
                    let package_version = parse_package_version(&package)?;

                    swimlane_client
                        .install_pip_package(&package, Some(package_version))
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
        Command::Migrate {
            migration_type,
            target_url,
            target_pat,
            dry_run,
            auto_approve: _,
        } => {
            let target_swimlane_client = SwimlaneClient::new(target_url, target_pat);

            let migration_type = migration_type.unwrap_or(Migrate::All);

            handle_migrate(
                swimlane_client,
                target_swimlane_client,
                migration_type,
                dry_run,
            )
            .await?
        }
    }
    Ok(())
}
