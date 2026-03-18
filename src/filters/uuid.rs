use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * UUID v4 generation filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_uuid(_value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let id = uuid::Uuid::new_v4().to_string();
    tera::to_value(id).map_err(|err| Error::msg(format!("Failed to serialize uuid value: {err}")))
}

#[cfg(test)]
mod tests {
    use super::filter_uuid;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn uuid_returns_valid_length() {
        let input = Value::String(String::new());
        let args = HashMap::new();
        let result = filter_uuid(&input, &args).unwrap();
        let uuid_str = result.as_str().unwrap();
        assert_eq!(uuid_str.len(), 36);
    }

    #[test]
    fn uuid_returns_unique_values() {
        let input = Value::String(String::new());
        let args = HashMap::new();
        let first = filter_uuid(&input, &args).unwrap();
        let second = filter_uuid(&input, &args).unwrap();
        assert_ne!(first, second);
    }
}
