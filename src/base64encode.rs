use std::collections::HashMap;

use base64::{Engine as _, engine::general_purpose};
use serde_json::Value;
use tera::Error;

pub fn filter_base64_encode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let encoded_value = general_purpose::STANDARD.encode(value.as_str().unwrap_or(""));

    Ok(tera::to_value(encoded_value).unwrap())
}