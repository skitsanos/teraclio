use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * JSON encode filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_json_encode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let result = serde_json::to_string_pretty(value)
        .map_err(|err| Error::msg(format!("Failed to serialize value to JSON: {err}")))?;
    tera::to_value(result)
        .map_err(|err| Error::msg(format!("Failed to serialize encoded value: {err}")))
}

/**
 * YAML encode filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_yaml_encode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let result = serde_yaml::to_string(value)
        .map_err(|err| Error::msg(format!("Failed to serialize value to YAML: {err}")))?;
    tera::to_value(result)
        .map_err(|err| Error::msg(format!("Failed to serialize encoded value: {err}")))
}

#[cfg(test)]
mod tests {
    use super::{filter_json_encode, filter_yaml_encode};
    use serde_json::{json, Value};
    use std::collections::HashMap;

    #[test]
    fn json_encode_returns_pretty_json() {
        let input = json!({"key": "value"});
        let args = HashMap::new();
        let result = filter_json_encode(&input, &args).unwrap();
        let expected = serde_json::to_string_pretty(&json!({"key": "value"})).unwrap();
        assert_eq!(result, Value::String(expected));
    }

    #[test]
    fn yaml_encode_returns_yaml_string() {
        let input = json!({"key": "value"});
        let args = HashMap::new();
        let result = filter_yaml_encode(&input, &args).unwrap();
        let expected = serde_yaml::to_string(&json!({"key": "value"})).unwrap();
        assert_eq!(result, Value::String(expected));
    }
}
