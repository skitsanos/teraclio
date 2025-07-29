use std::collections::HashMap;
use serde_json::Value;
use tera::Error;
use html_escape;

/**
 * HTML escape filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_html_escape(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let escaped = html_escape::encode_text(input_str);
    Ok(tera::to_value(escaped.into_owned()).unwrap())
}

/**
 * HTML unescape filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_html_unescape(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let unescaped = html_escape::decode_html_entities(input_str);
    Ok(tera::to_value(unescaped.into_owned()).unwrap())
}

/**
 * XML escape filter for Tera templates (same as HTML for basic entities)
 * @author: skitsanos
 */
pub fn filter_xml_escape(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value.as_str().unwrap_or("");
    let escaped = html_escape::encode_text(input_str);
    Ok(tera::to_value(escaped.into_owned()).unwrap())
}