use std::collections::HashMap;

use serde_json::Value;
use tera::Error;

pub fn filter_str_to_bytes(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    if let Value::String(string) = value {
        let bytes: Vec<Value> = string.bytes().map(|b| Value::Number(serde_json::Number::from(b))).collect();
        Ok(tera::to_value(bytes).unwrap())
    } else {
        Err(Error::msg("Invalid input: expected a string"))
    }
}
