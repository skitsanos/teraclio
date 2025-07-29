use crate::error::{Result, TeraclioError};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum InputFormat {
    Json,
    Yaml,
    Toml,
}

impl InputFormat {
    pub fn detect_from_extension(path: &Path) -> Self {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            match extension.to_lowercase().as_str() {
                "yaml" | "yml" => InputFormat::Yaml,
                "toml" => InputFormat::Toml,
                _ => InputFormat::Json,
            }
        } else {
            InputFormat::Json
        }
    }

    pub fn from_string(format_str: &str) -> Result<Self> {
        match format_str.to_lowercase().as_str() {
            "json" => Ok(InputFormat::Json),
            "yaml" | "yml" => Ok(InputFormat::Yaml),
            "toml" => Ok(InputFormat::Toml),
            _ => Err(TeraclioError::InvalidInput(format!(
                "Unsupported input format: {format_str}. Supported formats: json, yaml, toml"
            ))),
        }
    }
}

/**
 * Parse a data file into a serde_json::Value
 * @author: skitsanos
 *
 * # Arguments
 *
 * * `source_path` - Path to the data file to parse
 * * `format` - Optional format specification, auto-detected if None
 *
 * # Returns
 *
 * A serde_json::Value containing the parsed data
 */
pub fn parse_data_source(source_path: &PathBuf, format: Option<&str>) -> Result<Value> {
    if !source_path.exists() {
        return Err(TeraclioError::InvalidInput(format!(
            "Data source file does not exist: {}",
            source_path.display()
        )));
    }

    let contents = fs::read_to_string(source_path)?;

    if contents.trim().is_empty() {
        return Err(TeraclioError::InvalidInput(
            "Data source file is empty".to_string(),
        ));
    }

    let input_format = if let Some(fmt) = format {
        InputFormat::from_string(fmt)?
    } else {
        InputFormat::detect_from_extension(source_path)
    };

    let value = match input_format {
        InputFormat::Json => serde_json::from_str(&contents).map_err(TeraclioError::JsonError)?,
        InputFormat::Yaml => serde_yaml::from_str(&contents)
            .map_err(|e| TeraclioError::InvalidInput(format!("YAML parsing error: {e}")))?,
        InputFormat::Toml => toml::from_str(&contents)
            .map_err(|e| TeraclioError::InvalidInput(format!("TOML parsing error: {e}")))?,
    };

    Ok(value)
}
