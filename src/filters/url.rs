use serde_json::Value;
use std::collections::HashMap;
use tera::Error;
use urlencoding;

/**
 * URL encode filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_url_encode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let encoded = urlencoding::encode(input_str);
    Ok(tera::to_value(encoded.into_owned()).unwrap())
}

/**
 * URL decode filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_url_decode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    match urlencoding::decode(input_str) {
        Ok(decoded) => Ok(tera::to_value(decoded.into_owned()).unwrap()),
        Err(_) => Err(Error::msg("Failed to decode URL: invalid encoding")),
    }
}
