use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn file_path_to_hashmap(
    file_path: &impl AsRef<Path>,
) -> Result<HashMap<String, String>, UploadRequirementsError> {
    if !file_path.as_ref().exists() {
        return Err(UploadRequirementsError::FileNotFound {
            source: io::Error::new(io::ErrorKind::NotFound, "File not found"),
        });
    }

    let file = File::open(file_path.as_ref()).unwrap();
    let lines = BufReader::new(file).lines();

    let mut requirements: HashMap<String, String> = HashMap::new();

    let mut line_number = 0;

    for line in lines {
        let line = line.unwrap();
        line_number += 1;
        let line = line.trim();
        if line.starts_with('#') {
            println!(
                "Skipping line {} as it's a comment: '{}'",
                line_number, line
            );
            continue;
        }
        let package = line.split("==").collect::<Vec<_>>();
        if package.len() != 2 {
            return Err(UploadRequirementsError::InvalidFormat {
                source: InvalidFormat,
            });
        }

        if requirements.contains_key(package[0]) {
            return Err(UploadRequirementsError::InvalidFormat {
                source: InvalidFormat,
            });
        } else {
            requirements.insert(
                package[0].to_string().to_lowercase(),
                package[1].to_string().to_lowercase(),
            );
        }
    }

    Ok(requirements)
}

#[derive(Debug, Clone)]
pub struct InvalidFormat;

#[derive(Debug)]
pub enum UploadRequirementsError {
    FileNotFound { source: io::Error },
    InvalidFormat { source: InvalidFormat },
}
