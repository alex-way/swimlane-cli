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
                std::fs::create_dir(&path)
                    .expect(format!("Could not create path: '{}'", path.display()).as_str());
            }

            let applications = swimlane_client
                .get_applications_light()
                .await
                .expect("Could not get applications");

            let mut handles = vec![];

            for application in applications {
                // todo: remove requirement for cloning
                let sw = swimlane_client.clone();
                let path = path.clone();
                let handle = tokio::spawn(async move {
                    println!("Downloading tasks for application: '{}'", application.name);
                    sw.download_tasks_for_application(&application, &path)
                        .await
                        .unwrap();
                    println!(
                        "Finished downloading tasks for application: '{}'",
                        application.name
                    );
                });
                handles.push(handle);
            }

            // todo: Download common tasks to /common folder

            handles.push(tokio::spawn(async move {
                println!("Downloading common tasks");
                swimlane_client.download_common_tasks(&path).await.unwrap();
                println!("Finished downloading common tasks");
            }));

            for handle in handles {
                handle.await.unwrap();
            }
        }
    }
}
