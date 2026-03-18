use serde_json::Value;
use std::collections::HashMap;
use tera::Error;
use urlencoding;

/**
 * URL encode filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_url_encode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for url_encode"))?;
    let encoded = urlencoding::encode(input_str);
    tera::to_value(encoded.into_owned())
        .map_err(|err| Error::msg(format!("Failed to serialize encoded value: {err}")))
}

/**
 * URL decode filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_url_decode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for url_decode"))?;
    match urlencoding::decode(input_str) {
        Ok(decoded) => tera::to_value(decoded.into_owned())
            .map_err(|err| Error::msg(format!("Failed to serialize decoded value: {err}"))),
        Err(_) => Err(Error::msg("Failed to decode URL: invalid encoding")),
    }
}
