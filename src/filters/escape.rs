use html_escape;
use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * HTML escape filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_html_escape(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for html_escape"))?;
    let escaped = html_escape::encode_text(input_str);
    tera::to_value(escaped.into_owned())
        .map_err(|err| Error::msg(format!("Failed to serialize escaped value: {err}")))
}

/**
 * HTML unescape filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_html_unescape(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for html_unescape"))?;
    let unescaped = html_escape::decode_html_entities(input_str);
    tera::to_value(unescaped.into_owned())
        .map_err(|err| Error::msg(format!("Failed to serialize unescaped value: {err}")))
}

/**
 * XML escape filter for Tera templates (same as HTML for basic entities)
 * @author: skitsanos
 */
pub fn filter_xml_escape(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for xml_escape"))?;
    let escaped = html_escape::encode_text(input_str);
    tera::to_value(escaped.into_owned())
        .map_err(|err| Error::msg(format!("Failed to serialize escaped value: {err}")))
}
