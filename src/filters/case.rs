use convert_case::{Case, Casing};
use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * Convert string to snake_case
 * @author: skitsanos
 */
pub fn filter_snake_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let converted = input_str.to_case(Case::Snake);
    Ok(tera::to_value(converted).unwrap())
}

/**
 * Convert string to kebab-case
 * @author: skitsanos
 */
pub fn filter_kebab_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let converted = input_str.to_case(Case::Kebab);
    Ok(tera::to_value(converted).unwrap())
}

/**
 * Convert string to camelCase
 * @author: skitsanos
 */
pub fn filter_camel_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let converted = input_str.to_case(Case::Camel);
    Ok(tera::to_value(converted).unwrap())
}

/**
 * Convert string to PascalCase
 * @author: skitsanos
 */
pub fn filter_pascal_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let converted = input_str.to_case(Case::Pascal);
    Ok(tera::to_value(converted).unwrap())
}

/**
 * Convert string to slug (URL-friendly)
 * @author: skitsanos
 */
pub fn filter_slug(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let converted = input_str.to_case(Case::Kebab).to_lowercase();
    Ok(tera::to_value(converted).unwrap())
}
