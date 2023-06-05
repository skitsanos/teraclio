use std::collections::HashMap;
use serde_json::Value;
use tera::Error;

pub fn filter_bytes_to_str(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    if let Value::Array(bytes) = value {
        let string = bytes
            .iter()
            .filter_map(|byte| byte.as_u64().map(|b| b as u8))
            .collect::<Vec<u8>>();
        let result = String::from_utf8_lossy(&string).into_owned();
        Ok(tera::to_value(result).unwrap())
    } else {
        Err(Error::msg("Invalid input: expected an array of bytes"))
    }
}
