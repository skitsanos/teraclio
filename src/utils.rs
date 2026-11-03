use crate::error::{Result, TeraclioError};
use clap::ValueEnum;
use serde_json::Value;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum InputFormat {
    Json,
    #[value(name = "yaml", alias = "yml")]
    Yaml,
    Toml,
}

impl InputFormat {
    pub fn detect_from_extension(path: &Path) -> Option<Self> {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            match extension.to_lowercase().as_str() {
                "yaml" | "yml" => Some(InputFormat::Yaml),
                "toml" => Some(InputFormat::Toml),
                "json" => Some(InputFormat::Json),
                _ => None,
            }
        } else {
            Some(InputFormat::Json)
        }
    }
}

/**
 * Parse data source from a file path or stdin ("-") into a serde_json::Value
 * @author: skitsanos
 *
 * # Arguments
 *
 * * `source` - Data source path or "-" for stdin
 * * `format` - Optional format specification, auto-detected for files if omitted
 *
 * # Returns
 *
 * A serde_json::Value containing the parsed data
 */
pub fn parse_data_source(source: &str, format: Option<InputFormat>) -> Result<Value> {
    let (contents, input_format) = if source == "-" {
        let mut input = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut input)?;

        let input_format = format.ok_or_else(|| {
            TeraclioError::InvalidInput(
                "When reading from stdin, --format must be specified (json, yaml, or toml)."
                    .to_string(),
            )
        })?;

        (input, input_format)
    } else {
        let source_path = PathBuf::from(source);
        if !source_path.exists() {
            return Err(TeraclioError::InvalidInput(format!(
                "Data source file does not exist: {}",
                source_path.display()
            )));
        }

        let contents = fs::read_to_string(&source_path)?;
        let input_format = format
            .or_else(|| InputFormat::detect_from_extension(&source_path))
            .ok_or_else(|| {
                TeraclioError::InvalidInput(format!(
                    "Unsupported input format for file '{}'. Supported formats: json, yaml, toml. \
                     Provide --format explicitly.",
                    source_path.display()
                ))
            })?;

        (contents, input_format)
    };

    if contents.trim().is_empty() {
        return Err(TeraclioError::InvalidInput(
            "Data source file is empty".to_string(),
        ));
    }

    let value = match input_format {
        InputFormat::Json => serde_json::from_str(&contents).map_err(TeraclioError::JsonError)?,
        InputFormat::Yaml => noyalib::compat::serde_yaml::from_str(&contents)
            .map_err(|e| TeraclioError::InvalidInput(format!("YAML parsing error: {e}")))?,
        InputFormat::Toml => toml::from_str::<toml::Table>(&contents)
            .map(|table| toml_to_json(toml::Value::Table(table)))
            .map_err(|e| TeraclioError::InvalidInput(format!("TOML parsing error: {e}")))?,
    };

    Ok(value)
}

/**
 * Convert TOML to JSON explicitly. Deserializing TOML straight into a
 * serde_json::Value turns datetimes into `{"$__toml_private_datetime": ...}`
 * objects, which templates cannot print or pass to `date_format`. Here they
 * become their RFC 3339 / ISO 8601 string form instead.
 * @author: skitsanos
 */
fn toml_to_json(value: toml::Value) -> Value {
    match value {
        toml::Value::String(s) => Value::String(s),
        toml::Value::Integer(i) => Value::from(i),
        toml::Value::Float(f) => serde_json::Number::from_f64(f)
            .map(Value::Number)
            // JSON has no NaN/inf; keep TOML's spelling rather than dropping it
            .unwrap_or_else(|| Value::String(f.to_string())),
        toml::Value::Boolean(b) => Value::Bool(b),
        toml::Value::Datetime(dt) => Value::String(dt.to_string()),
        toml::Value::Array(items) => Value::Array(items.into_iter().map(toml_to_json).collect()),
        toml::Value::Table(table) => Value::Object(
            table
                .into_iter()
                .map(|(key, value)| (key, toml_to_json(value)))
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_data_source, InputFormat};
    use crate::error::TeraclioError;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_path(suffix: &str) -> std::path::PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time before unix epoch")
            .as_nanos();
        let mut path = std::env::temp_dir();
        if suffix.is_empty() {
            path.push(format!("teraclio-test-{now}"));
        } else {
            path.push(format!("teraclio-test-{now}.{suffix}"));
        }
        path
    }

    fn write_temp_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = unique_path(name);
        std::fs::write(&path, contents).expect("write temp test file");
        path
    }

    #[test]
    fn parses_json_without_extension() {
        let path = write_temp_file("", r#"{"name":"value"}"#);
        let result = parse_data_source(path.to_str().expect("utf8 path"), None).expect("parse");
        assert_eq!(result["name"], "value");
        std::fs::remove_file(path).expect("cleanup");
    }

    #[test]
    fn rejects_unknown_extension_without_format() {
        let path = write_temp_file("cfg.txt", r#"{"name":"value"}"#);
        let err = parse_data_source(path.to_str().expect("utf8 path"), None).expect_err("error");
        assert!(matches!(err, TeraclioError::InvalidInput(_)));
        if let TeraclioError::InvalidInput(msg) = err {
            assert!(msg.contains("Unsupported input format for file"), "{msg}");
        }
        std::fs::remove_file(path).expect("cleanup");
    }

    #[test]
    fn parses_unknown_extension_with_explicit_format() {
        let path = write_temp_file("cfg.txt", r#"name = "foo""#);
        let result = parse_data_source(path.to_str().expect("utf8 path"), Some(InputFormat::Toml))
            .expect("parse");
        assert_eq!(result["name"], "foo");
        std::fs::remove_file(path).expect("cleanup");
    }

    #[test]
    fn toml_datetimes_become_strings() {
        let path = write_temp_file(
            "toml",
            "created = 2024-01-15T10:30:00Z\nday = 2024-01-15\nlocal = 2024-01-15T10:30:00\nat = 10:30:00\n[nested]\nwhen = [2024-02-01]\n",
        );
        let result = parse_data_source(path.to_str().expect("utf8 path"), None).expect("parse");
        assert_eq!(result["created"], "2024-01-15T10:30:00Z");
        assert_eq!(result["day"], "2024-01-15");
        assert_eq!(result["local"], "2024-01-15T10:30:00");
        assert_eq!(result["at"], "10:30:00");
        assert_eq!(result["nested"]["when"][0], "2024-02-01");
        std::fs::remove_file(path).expect("cleanup");
    }

    #[test]
    fn toml_non_finite_floats_are_kept_as_strings() {
        let path = write_temp_file("toml", "a = inf\nb = nan\nc = 1.5\n");
        let result = parse_data_source(path.to_str().expect("utf8 path"), None).expect("parse");
        assert_eq!(result["a"], "inf");
        assert_eq!(result["b"], "NaN");
        assert_eq!(result["c"], 1.5);
        std::fs::remove_file(path).expect("cleanup");
    }
}
