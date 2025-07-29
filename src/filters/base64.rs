use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * Base64 encode filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_base64_encode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let encoded_value = general_purpose::STANDARD.encode(input_str);
    Ok(tera::to_value(encoded_value).unwrap())
}

/**
 * Base64 decode filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_base64_decode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");

    match general_purpose::STANDARD.decode(input_str) {
        Ok(decoded_bytes) => Ok(tera::to_value(decoded_bytes).unwrap()),
        Err(_) => Err(Error::msg("Failed to decode Base64: invalid input")),
    }
}
