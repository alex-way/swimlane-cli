use clap::{arg, Parser, Subcommand};
use std::path::PathBuf;
use swimlane::SwimlaneClient;

/// A fictional versioning CLI
#[derive(Debug, Parser)]
#[command(name = "swimlane-cli")]
#[command(about = "A simple CLI for interacting with Swimlane", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long)]
    source_swimlane_url: String,

    #[arg(long)]
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
    /// Uploads a specified requirements.txt file to all custom python tasks
    #[command(arg_required_else_help = true)]
    UploadRequirements {
        /// Where the tasks will be downloaded to
        path: PathBuf,
    },
    /// Migrates data from the source Swimlane server to the destination Swimlane server
    Migrate {
        #[clap(subcommand)]
        migration_type: Migrate,
        #[arg(long)]
        destination_swimlane_url: String,
        #[arg(long)]
        destination_swimlane_pat: String,
    },
}

#[derive(Debug, Subcommand)]
enum Migrate {
    /// Migrates all users from the source Swimlane server to the destination Swimlane server
    #[command()]
    Users,
    /// Migrates the specified user from the source Swimlane server to the destination Swimlane server
    #[command(arg_required_else_help = true)]
    User { user_id: String },
    /// Migrates all groups from the source Swimlane server to the destination Swimlane server
    #[command()]
    Groups,
    /// Migrates the specified group from the source Swimlane server to the destination Swimlane server
    #[command(arg_required_else_help = true)]
    Group { group_id: String },
    /// Migrates all roles from the source Swimlane server to the destination Swimlane server
    #[command()]
    Roles,
    /// Migrates the specified role from the source Swimlane server to the destination Swimlane server
    #[command(arg_required_else_help = true)]
    Role { group_id: String },
    /// Migrates all applications from the source Swimlane server to the destination Swimlane server
    #[command()]
    Applications,
    /// Migrates the specified application from the source Swimlane server to the destination Swimlane server
    #[command(arg_required_else_help = true)]
    Application { application_name: String },

    /// Migrates all possible content from the source Swimlane server to the destination Swimlane server
    #[command()]
    All,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let swimlane_client = SwimlaneClient::new(args.source_swimlane_url, args.source_swimlane_pat);

    swimlane_client.health_ping().await.unwrap();

    match args.command {
        Commands::DownloadPythonTasks { path } => {
            swimlane_client.download_python_tasks(&path).await.unwrap();
        }
        Commands::UploadRequirements { path } => {
            swimlane_client
                .upload_python_requirements(&path)
                .await
                .unwrap();
        }
        Commands::Migrate {
            migration_type,
            destination_swimlane_url,
            destination_swimlane_pat,
        } => match migration_type {
            Migrate::Users {} => {
                println!("{}, {}", destination_swimlane_url, destination_swimlane_pat);
            }
            Migrate::User { user_id: _ } => {
                todo!();
            }
            Migrate::Groups {} => {
                todo!();
            }
            Migrate::Group { group_id: _ } => {
                todo!();
            }
            Migrate::Roles {} => {
                todo!();
            }
            Migrate::Role { group_id: _ } => {
                todo!();
            }
            Migrate::Applications {} => {
                todo!();
            }
            Migrate::Application {
                application_name: _,
            } => {
                todo!();
            }
            Migrate::All {} => {
                todo!();
            }
        },
    }
}
