use convert_case::{Case, Casing};
use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * Convert string to snake_case
 * @author: skitsanos
 */
pub fn filter_snake_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for snake_case"))?;
    let converted = input_str.to_case(Case::Snake);
    tera::to_value(converted).map_err(|err| Error::msg(format!("Failed to serialize value: {err}")))
}

/**
 * Convert string to kebab-case
 * @author: skitsanos
 */
pub fn filter_kebab_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for kebab_case"))?;
    let converted = input_str.to_case(Case::Kebab);
    tera::to_value(converted).map_err(|err| Error::msg(format!("Failed to serialize value: {err}")))
}

/**
 * Convert string to camelCase
 * @author: skitsanos
 */
pub fn filter_camel_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for camel_case"))?;
    let converted = input_str.to_case(Case::Camel);
    tera::to_value(converted).map_err(|err| Error::msg(format!("Failed to serialize value: {err}")))
}

/**
 * Convert string to PascalCase
 * @author: skitsanos
 */
pub fn filter_pascal_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for pascal_case"))?;
    let converted = input_str.to_case(Case::Pascal);
    tera::to_value(converted).map_err(|err| Error::msg(format!("Failed to serialize value: {err}")))
}

/**
 * Convert string to slug (URL-friendly)
 * @author: skitsanos
 */
pub fn filter_slug(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for slug"))?;
    let converted = input_str.to_case(Case::Kebab).to_lowercase();
    tera::to_value(converted).map_err(|err| Error::msg(format!("Failed to serialize value: {err}")))
}
