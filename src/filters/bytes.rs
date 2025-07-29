use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * Convert bytes array to string filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_bytes_to_str(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    if let Value::Array(bytes) = value {
        let byte_vec: Vec<u8> = bytes
            .iter()
            .filter_map(|byte| byte.as_u64().map(|b| b as u8))
            .collect();

        let result = String::from_utf8_lossy(&byte_vec).into_owned();
        Ok(tera::to_value(result).unwrap())
    } else {
        Err(Error::msg("Invalid input: expected an array of bytes"))
    }
}

/**
 * Convert string to bytes array filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_str_to_bytes(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    if let Value::String(string) = value {
        let bytes: Vec<Value> = string
            .bytes()
            .map(|b| Value::Number(serde_json::Number::from(b)))
            .collect();
        Ok(tera::to_value(bytes).unwrap())
    } else {
        Err(Error::msg("Invalid input: expected a string"))
    }
}
