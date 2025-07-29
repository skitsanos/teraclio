use std::collections::HashMap;
use serde_json::Value;
use tera::Error;
use sha1::Sha1;
use sha2::{Sha256, Digest};

/**
 * MD5 hash filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_md5(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let result = md5::compute(input_str.as_bytes());
    let hash_string = format!("{:x}", result);
    Ok(tera::to_value(hash_string).unwrap())
}

/**
 * SHA1 hash filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_sha1(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let mut hasher = Sha1::new();
    Digest::update(&mut hasher, input_str.as_bytes());
    let result = hasher.finalize();
    let hash_string = format!("{:x}", result);
    Ok(tera::to_value(hash_string).unwrap())
}

/**
 * SHA256 hash filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_sha256(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, input_str.as_bytes());
    let result = hasher.finalize();
    let hash_string = format!("{:x}", result);
    Ok(tera::to_value(hash_string).unwrap())
}