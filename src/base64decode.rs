use std::collections::HashMap;

use base64::{Engine as _, engine::general_purpose};
use serde_json::Value;
use tera::Error;

pub fn filter_base64_decode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let decoded_value = general_purpose::STANDARD.decode(value.as_str().unwrap_or(""));
    match decoded_value {
        Ok(decoded_str) => Ok(tera::to_value(decoded_str).unwrap()),
        Err(_) => Err(Error::msg("Failed to decode Base64")),
    }
}