use crate::error::{Result, TeraclioError};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/**
 * Parse a JSON file into a serde_json::Value
 * @author: skitsanos
 *
 * # Arguments
 *
 * * `source_path` - Path to the JSON file to parse
 *
 * # Returns
 *
 * A serde_json::Value containing the parsed JSON data
 */
pub fn parse_json_source(source_path: &PathBuf) -> Result<Value> {
    if !source_path.exists() {
        return Err(TeraclioError::InvalidInput(format!(
            "JSON source file does not exist: {}",
            source_path.display()
        )));
    }

    let contents = fs::read_to_string(source_path)?;

    if contents.trim().is_empty() {
        return Err(TeraclioError::InvalidInput(
            "JSON source file is empty".to_string(),
        ));
    }

    let json_value: Value = serde_json::from_str(&contents)?;
    Ok(json_value)
}
