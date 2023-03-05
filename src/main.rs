use crate::swimlane::SwimlaneClient;
use clap::{arg, Parser, Subcommand};
use std::path::PathBuf;

pub mod swimlane;

/// A fictional versioning CLI
#[derive(Debug, Parser)]
#[command(name = "swimmy")]
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
    }
}
