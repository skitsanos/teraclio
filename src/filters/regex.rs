use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * Replace all occurrences matching a regex pattern in a string
 * @author: skitsanos
 */
pub fn filter_regex_replace(value: &Value, args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for regex_replace"))?;

    let pattern = args
        .get("pattern")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::msg("Missing required argument: pattern"))?;

    let replacement = args
        .get("replacement")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::msg("Missing required argument: replacement"))?;

    let regex =
        Regex::new(pattern).map_err(|err| Error::msg(format!("Invalid regex pattern: {err}")))?;

    let result = regex.replace_all(input_str, replacement);
    tera::to_value(result.as_ref())
        .map_err(|err| Error::msg(format!("Failed to serialize value: {err}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_replacement() {
        let value = Value::String("hello 123 world 456".to_string());
        let mut args = HashMap::new();
        args.insert("pattern".to_string(), Value::String(r"\d+".to_string()));
        args.insert("replacement".to_string(), Value::String("NUM".to_string()));

        let result = filter_regex_replace(&value, &args).unwrap();
        assert_eq!(result, Value::String("hello NUM world NUM".to_string()));
    }

    #[test]
    fn test_missing_pattern_arg() {
        let value = Value::String("hello".to_string());
        let mut args = HashMap::new();
        args.insert("replacement".to_string(), Value::String("x".to_string()));

        let result = filter_regex_replace(&value, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("pattern"));
    }

    #[test]
    fn test_missing_replacement_arg() {
        let value = Value::String("hello".to_string());
        let mut args = HashMap::new();
        args.insert("pattern".to_string(), Value::String(r"\d+".to_string()));

        let result = filter_regex_replace(&value, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("replacement"));
    }

    #[test]
    fn test_invalid_regex() {
        let value = Value::String("hello".to_string());
        let mut args = HashMap::new();
        args.insert("pattern".to_string(), Value::String("[invalid".to_string()));
        args.insert("replacement".to_string(), Value::String("x".to_string()));

        let result = filter_regex_replace(&value, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid regex"));
    }
}
