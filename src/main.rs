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
            // Create the path if it doesnt exist
            if !path.exists() {
                std::fs::create_dir(&path).expect("Could not create directory");
            }
            let applications = swimlane_client
                .get_applications_light()
                .await
                .expect("Could not get applications");

            let mut handles = vec![];

            for application in applications {
                // todo: remove requirement for cloning here
                let sw = swimlane_client.clone();
                let path = path.clone();
                // todo: directly spawn download tasks for application
                let handle = tokio::spawn(async move {
                    sw.download_tasks_for_application(&application, &path).await
                });
                handles.push(handle);
            }

            for handle in handles {
                // todo: deduplicate expect
                handle
                    .await
                    .expect("Could not download tasks")
                    .expect("Could not download tasks");
            }
        }
    }
}
