use crate::error::SwimlaneCliError;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use swimlane::apps::LightApplication;
use swimlane::{tasks::Task, SwimlaneClient};
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

async fn download_tasks_to_folder(
    folder: &PathBuf,
    tasks: Vec<Task>,
) -> Result<(), SwimlaneCliError> {
    let downloadable_tasks = tasks
        .into_iter()
        .filter(|t| t.action.script.is_some())
        .collect::<Vec<_>>();

    if !folder.exists() && !downloadable_tasks.is_empty() {
        create_dir(folder)
            .unwrap_or_else(|_| panic!("Could not create folder: '{}'", folder.display()));
    }

    let mut handles = vec![];

    for task in downloadable_tasks {
        let folder = folder.clone();
        let handle = tokio::spawn(async move {
            println!("Downloading task: '{}'", task.name);
            save_task(&task, &folder)
                .await
                .unwrap_or_else(|_| panic!("Could not save task: '{}'", task.name));
            println!("Downloaded task: '{}'", task.name);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}

pub async fn download_tasks_for_application(
    swimlane_client: &SwimlaneClient,
    application: &LightApplication,
    path: &impl AsRef<Path>,
) -> Result<(), SwimlaneCliError> {
    println!("Downloading tasks for application: '{}'", application.name);
    let tasks = swimlane_client
        .get_tasks_for_application(&application.id)
        .await?;

    let folder = path.as_ref().join(&application.name);
    download_tasks_to_folder(&folder, tasks).await
}

pub async fn download_common_tasks(
    swimlane_client: &SwimlaneClient,
    path: &impl AsRef<Path>,
) -> Result<(), SwimlaneCliError> {
    println!("Downloading common tasks");
    let tasks = swimlane_client.get_common_tasks().await?;

    let folder = path.as_ref().join("common");
    download_tasks_to_folder(&folder, tasks).await
}

async fn save_task(task: &Task, path: &impl AsRef<Path>) -> io::Result<()> {
    match &task.action.script {
        Some(script) => {
            let file_path = path.as_ref().join(format!("{}.py", task.name));
            let mut file = File::create(&file_path).await?;
            file.write_all(script.as_bytes())
                .await
                .unwrap_or_else(|_| panic!("Could not write to file: '{}'", &file_path.display()));
            Ok(())
        }
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Task '{}' has no script", task.name),
        )),
    }
}

/// Saves all python tasks to the specified path in the format '{application_name}/{task_name}.py'
pub async fn save_python_tasks(
    swimlane_client: &SwimlaneClient,
    path: &impl AsRef<Path>,
    app: &Option<String>,
) -> Result<(), SwimlaneCliError> {
    // Create the path if it doesnt exist
    if !path.as_ref().exists() {
        std::fs::create_dir(path)
            .unwrap_or_else(|_| panic!("Could not create path: '{}'", &path.as_ref().display()));
    }

    let mut applications = swimlane_client
        .get_applications_light()
        .await
        .expect("Could not get applications");

    // If an application is specified, filter the applications to only include that application
    if let Some(app) = app {
        applications = applications
            .into_iter()
            .filter(|a| a.name == *app)
            .collect::<Vec<_>>();
    }

    let mut handles = vec![];

    for application in applications.clone() {
        let sw = swimlane_client.clone();
        let path = path.as_ref().to_path_buf();
        let handle = tokio::spawn(async move {
            download_tasks_for_application(&sw, &application, &path)
                .await
                .unwrap_or_else(|_| {
                    panic!(
                        "Could not download tasks for application: '{}'",
                        application.name
                    )
                });
        });
        handles.push(handle);
    }

    // if common in applications

    if app.is_none() || app == &Some("common".to_string()) {
        let sw = swimlane_client.clone();
        let path = path.as_ref().to_path_buf();
        let handle = tokio::spawn(async move {
            download_common_tasks(&sw, &path)
                .await
                .unwrap_or_else(|_| panic!("Could not download common tasks"));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}
