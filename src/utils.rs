use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde_json::Value;

/**
 * Parse a JSON file into a serde_json::Value
 *
 * # Arguments
 *
 * * `source_path` - Path to the JSON file to parse
 *
 * # Returns
 *
 * A serde_json::Value containing the parsed JSON data
 */
pub fn parse_json_source(source_path: &PathBuf) -> Result<Value, Box<dyn std::error::Error>> {
    // Read the contents of the JSON file
    let mut file = File::open(source_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON contents into a serde_json::Value
    let json_value: Value = serde_json::from_str(&contents)?;

    Ok(json_value)
}
