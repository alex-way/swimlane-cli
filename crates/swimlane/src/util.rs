use serde::Deserialize;

use crate::error::{SwimlaneClientError, UploadRequirementsError};
use crate::SwimlaneClient;
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

impl SwimlaneClient {
    pub async fn get_paginated_items<T>(&self, url: &str) -> Result<Vec<T>, SwimlaneClientError>
    where
        T: for<'de> Deserialize<'de> + Clone,
    {
        const MAX_ITEMS_PER_PAGE: usize = 100;
        // Weirdly page numbers are 0 indexed
        let mut page_number = 0;

        let mut items: Vec<T> = Vec::new();

        loop {
            let response = self
                .http_client
                .get(url)
                .query(&[("size", MAX_ITEMS_PER_PAGE), ("pageNumber", page_number)])
                .send()
                .await?;

            let payload: PagedResponse<T> = response.json().await?;

            let payload_length = payload.items.len();

            items.extend(payload.items);

            if payload_length < MAX_ITEMS_PER_PAGE {
                break;
            }

            page_number += 1;
        }
        Ok(items)
    }
}

/// A macro to create a serde enum with the following derived traits: Serialize, Deserialize, Debug, Clone, PartialEq
macro_rules! serde_enum {
    ($name:ident, { $($variant:ident),* $(,)? }) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum $name {
            $($variant),*
        }
    };
}
