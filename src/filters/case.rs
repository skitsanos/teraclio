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
        .ok_or_else(|| Error::message("Invalid input: expected a string for snake_case"))?;
    let converted = input_str.to_case(Case::Snake);
    serde_json::to_value(converted)
        .map_err(|err| Error::message(format!("Failed to serialize value: {err}")))
}

/**
 * Convert string to kebab-case
 * @author: skitsanos
 */
pub fn filter_kebab_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::message("Invalid input: expected a string for kebab_case"))?;
    let converted = input_str.to_case(Case::Kebab);
    serde_json::to_value(converted)
        .map_err(|err| Error::message(format!("Failed to serialize value: {err}")))
}

/**
 * Convert string to camelCase
 * @author: skitsanos
 */
pub fn filter_camel_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::message("Invalid input: expected a string for camel_case"))?;
    let converted = input_str.to_case(Case::Camel);
    serde_json::to_value(converted)
        .map_err(|err| Error::message(format!("Failed to serialize value: {err}")))
}

/**
 * Convert string to PascalCase
 * @author: skitsanos
 */
pub fn filter_pascal_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::message("Invalid input: expected a string for pascal_case"))?;
    let converted = input_str.to_case(Case::Pascal);
    serde_json::to_value(converted)
        .map_err(|err| Error::message(format!("Failed to serialize value: {err}")))
}

/**
 * Convert string to slug (URL-friendly)
 * @author: skitsanos
 */
pub fn filter_slug(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::message("Invalid input: expected a string for slug"))?;
    serde_json::to_value(slugify(input_str))
        .map_err(|err| Error::message(format!("Failed to serialize value: {err}")))
}

/**
 * Lowercase, split camelCase via kebab conversion, then collapse every run of
 * non-alphanumeric characters into a single '-' and trim leading/trailing dashes.
 * @author: skitsanos
 */
fn slugify(input: &str) -> String {
    let mut slug = String::with_capacity(input.len());
    for ch in input.to_case(Case::Kebab).to_lowercase().chars() {
        if ch.is_alphanumeric() {
            slug.push(ch);
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    slug
}

#[cfg(test)]
mod tests {
    use super::filter_slug;
    use serde_json::Value;
    use std::collections::HashMap;

    fn slug(input: &str) -> String {
        filter_slug(&Value::String(input.to_string()), &HashMap::new())
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn slug_strips_punctuation() {
        assert_eq!(slug("Hello World!"), "hello-world");
        assert_eq!(slug("  C++ & Rust: 2024 edition?  "), "c-rust-2024-edition");
        assert_eq!(slug("--already-slugged--"), "already-slugged");
    }

    #[test]
    fn slug_splits_camel_case_and_keeps_unicode_letters() {
        assert_eq!(slug("HelloWorld"), "hello-world");
        assert_eq!(slug("Café Münster"), "café-münster");
    }

    #[test]
    fn slug_of_only_punctuation_is_empty() {
        assert_eq!(slug("!!!"), "");
    }
}
