use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * Truncate a string to a given number of words
 * @author: skitsanos
 */
pub fn filter_truncate_words(value: &Value, args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for truncate_words"))?;

    let count = args.get("count").and_then(|v| v.as_u64()).ok_or_else(|| {
        Error::msg("Missing or invalid required argument: count (expected integer)")
    })? as usize;

    let end = args.get("end").and_then(|v| v.as_str()).unwrap_or("...");

    let words: Vec<&str> = input_str.split_whitespace().collect();

    if words.len() <= count {
        return tera::to_value(input_str)
            .map_err(|err| Error::msg(format!("Failed to serialize value: {err}")));
    }

    let truncated = format!("{}{}", words[..count].join(" "), end);
    tera::to_value(truncated).map_err(|err| Error::msg(format!("Failed to serialize value: {err}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_words() {
        let value = Value::String("The quick brown fox jumps over the lazy dog".to_string());
        let mut args = HashMap::new();
        args.insert("count".to_string(), Value::from(5));

        let result = filter_truncate_words(&value, &args).unwrap();
        assert_eq!(
            result,
            Value::String("The quick brown fox jumps...".to_string())
        );
    }

    #[test]
    fn test_no_truncation_when_short_enough() {
        let value = Value::String("Hello world".to_string());
        let mut args = HashMap::new();
        args.insert("count".to_string(), Value::from(5));

        let result = filter_truncate_words(&value, &args).unwrap();
        assert_eq!(result, Value::String("Hello world".to_string()));
    }

    #[test]
    fn test_custom_end_string() {
        let value = Value::String("The quick brown fox jumps over the lazy dog".to_string());
        let mut args = HashMap::new();
        args.insert("count".to_string(), Value::from(3));
        args.insert("end".to_string(), Value::String(" [more]".to_string()));

        let result = filter_truncate_words(&value, &args).unwrap();
        assert_eq!(result, Value::String("The quick brown [more]".to_string()));
    }
}
