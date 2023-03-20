use serde::Deserialize;

use crate::error::UploadRequirementsError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn file_path_to_hashmap(
    file_path: &impl AsRef<Path>,
) -> Result<HashMap<String, String>, UploadRequirementsError> {
    let file = File::open(file_path.as_ref()).map_err(UploadRequirementsError::FileNotFound)?;

    let lines = BufReader::new(file).lines();

    let mut requirements: HashMap<String, String> = HashMap::new();

    for (line_number, line) in lines.enumerate() {
        let line = line.map_err(UploadRequirementsError::FileNotFound)?;
        let line = line.trim();
        if line.starts_with('#') {
            println!(
                "Skipping line {} as it's a comment: '{}'",
                line_number + 1,
                line
            );
            continue;
        }
        let package = line.split("==").collect::<Vec<_>>();
        if package.len() != 2 {
            return Err(UploadRequirementsError::InvalidFormat {
                line_number: line_number + 1,
                line: line.to_owned(),
            });
        }

        let key = package[0].to_string().to_lowercase();
        let value = package[1].to_string().to_lowercase();
        if let Some(existing_value) = requirements.get(&key) {
            return Err(UploadRequirementsError::DuplicatePackage {
                key: key.clone(),
                line_number: line_number + 1,
                existing_value: existing_value.clone(),
                new_value: value,
            });
        }
        requirements.insert(key, value);
    }

    Ok(requirements)
}

#[derive(Debug, Deserialize)]
pub struct PagedResponse<T> {
    #[serde(rename = "totalCount")]
    pub total_count: usize,
    pub items: Vec<T>,
}
