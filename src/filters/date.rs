use chrono::{DateTime, NaiveDate, NaiveDateTime};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Write;
use tera::Error;

/**
 * Format a date/datetime string according to a chrono strftime format string
 * @author: skitsanos
 */
pub fn filter_date_format(value: &Value, args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::message("Invalid input: expected a string for date_format"))?;

    let format = args
        .get("format")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::message("Missing required argument: format"))?;

    // Most specific first: offset-aware RFC 3339, then local date-times
    // (with 'T' or space, optional fractional seconds), then a bare date.
    let formatted = if let Ok(dt) = DateTime::parse_from_rfc3339(input_str) {
        render(dt.format(format), format)?
    } else if let Some(dt) = ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%d %H:%M:%S%.f"]
        .iter()
        .find_map(|pattern| NaiveDateTime::parse_from_str(input_str, pattern).ok())
    {
        render(dt.format(format), format)?
    } else if let Ok(d) = NaiveDate::parse_from_str(input_str, "%Y-%m-%d") {
        render(d.format(format), format)?
    } else {
        return Err(Error::message(format!(
            "Unable to parse date string: '{input_str}'"
        )));
    };

    serde_json::to_value(formatted)
        .map_err(|err| Error::message(format!("Failed to serialize value: {err}")))
}

/**
 * Render a chrono format without panicking. `to_string()` panics when the
 * format string has an unknown specifier (e.g. `%Q`) or asks for data the
 * value lacks (e.g. `%z` on a local time); `write!` surfaces that as an error.
 * @author: skitsanos
 */
fn render(display: impl std::fmt::Display, format: &str) -> Result<String, Error> {
    let mut out = String::new();
    write!(out, "{display}").map_err(|_| {
        Error::message(format!(
            "Invalid date format '{format}': unknown specifier, or one that needs a timezone the input does not have"
        ))
    })?;
    Ok(out)
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

    fn format_with(input: &str, format: &str) -> Result<Value, Error> {
        let mut args = HashMap::new();
        args.insert("format".to_string(), Value::String(format.to_string()));
        filter_date_format(&Value::String(input.to_string()), &args)
    }

    #[test]
    fn test_invalid_format_specifier_is_an_error_not_a_panic() {
        let err = format_with("2024-06-15", "%Q").unwrap_err();
        assert!(err.to_string().contains("Invalid date format"), "{err}");
    }

    #[test]
    fn test_timezone_specifier_on_local_time_is_an_error() {
        assert!(format_with("2024-06-15T10:30:00", "%z").is_err());
    }

    #[test]
    fn test_local_datetime_variants() {
        for input in [
            "2024-06-15T10:30:00",
            "2024-06-15 10:30:00",
            "2024-06-15T10:30:00.123",
            "2024-06-15 10:30:00.123456",
        ] {
            assert_eq!(
                format_with(input, "%Y-%m-%d %H:%M").unwrap(),
                Value::String("2024-06-15 10:30".to_string()),
                "input: {input}"
            );
        }
    }
}
