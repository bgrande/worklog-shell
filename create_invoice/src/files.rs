use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use glob::glob;
use std::path::Path;
use std::fs;
use crate::structs::Position;

pub fn read_positions_until_date(base_path: &str, customer_id: &str, until_date: &NaiveDate) -> Result<Vec<Position>> {
    let mut positions = Vec::new();
    let glob_pattern = format!("{}/customers/{}/positions/**/*.json", base_path, customer_id);

    for entry in glob(&glob_pattern)? {
        match entry {
            Ok(path) => {
                // Skip directories and non-JSON files
                if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
                    continue;
                }

                // Try to get the date from the filename
                if let Some(file_date) = extract_date_from_path(&path) {
                    if file_date <= *until_date {
                        match read_position_file(&path) {
                            Ok(position) => positions.push(position),
                            Err(e) => log::warn!("Failed to read position file {:?}: {}", path, e),
                        }
                    }
                }
            }
            Err(e) => log::warn!("Failed to access path: {}", e),
        }
    }

    Ok(positions)
}

fn extract_date_from_path(path: &Path) -> Option<NaiveDate> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
}

fn read_position_file<P: AsRef<Path>>(path: P) -> Result<Position> {
    let content = fs::read_to_string(path)?;
    let position: Position = serde_json::from_str(&content)?;
    Ok(position)
}