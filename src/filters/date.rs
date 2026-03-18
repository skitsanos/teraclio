use chrono::{DateTime, NaiveDate, NaiveDateTime};
use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * Format a date/datetime string according to a chrono strftime format string
 * @author: skitsanos
 */
pub fn filter_date_format(value: &Value, args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for date_format"))?;

    let format = args
        .get("format")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("Missing required argument: format"))?;

    // Try RFC 3339 / ISO 8601 with timezone
    if let Ok(dt) = DateTime::parse_from_rfc3339(input_str) {
        let formatted = dt.format(format).to_string();
        return tera::to_value(formatted)
            .map_err(|err| Error::msg(format!("Failed to serialize value: {err}")));
    }

    // Try NaiveDateTime with "%Y-%m-%dT%H:%M:%S"
    if let Ok(dt) = NaiveDateTime::parse_from_str(input_str, "%Y-%m-%dT%H:%M:%S") {
        let formatted = dt.format(format).to_string();
        return tera::to_value(formatted)
            .map_err(|err| Error::msg(format!("Failed to serialize value: {err}")));
    }

    // Try NaiveDate with "%Y-%m-%d"
    if let Ok(d) = NaiveDate::parse_from_str(input_str, "%Y-%m-%d") {
        let formatted = d.format(format).to_string();
        return tera::to_value(formatted)
            .map_err(|err| Error::msg(format!("Failed to serialize value: {err}")));
    }

    Err(Error::msg(format!(
        "Unable to parse date string: '{input_str}'"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfc3339_input() {
        let value = Value::String("2024-06-15T10:30:00+02:00".to_string());
        let mut args = HashMap::new();
        args.insert("format".to_string(), Value::String("%Y-%m-%d".to_string()));

        let result = filter_date_format(&value, &args).unwrap();
        assert_eq!(result, Value::String("2024-06-15".to_string()));
    }

    #[test]
    fn test_date_only_input() {
        let value = Value::String("2024-06-15".to_string());
        let mut args = HashMap::new();
        args.insert("format".to_string(), Value::String("%d/%m/%Y".to_string()));

        let result = filter_date_format(&value, &args).unwrap();
        assert_eq!(result, Value::String("15/06/2024".to_string()));
    }

    #[test]
    fn test_invalid_date_error() {
        let value = Value::String("not-a-date".to_string());
        let mut args = HashMap::new();
        args.insert("format".to_string(), Value::String("%Y-%m-%d".to_string()));

        let result = filter_date_format(&value, &args);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unable to parse date string"));
    }
}
