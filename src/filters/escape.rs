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
        .ok_or_else(|| Error::message("Invalid input: expected a string for html_escape"))?;
    serde_json::to_value(escape_markup(input_str, "&#x27;"))
        .map_err(|err| Error::message(format!("Failed to serialize escaped value: {err}")))
}

/**
 * HTML unescape filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_html_unescape(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::message("Invalid input: expected a string for html_unescape"))?;
    let unescaped = html_escape::decode_html_entities(input_str);
    serde_json::to_value(unescaped.into_owned())
        .map_err(|err| Error::message(format!("Failed to serialize unescaped value: {err}")))
}

/**
 * XML escape filter for Tera templates (same as HTML for basic entities)
 * @author: skitsanos
 */
pub fn filter_xml_escape(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::message("Invalid input: expected a string for xml_escape"))?;
    serde_json::to_value(escape_markup(input_str, "&apos;"))
        .map_err(|err| Error::message(format!("Failed to serialize escaped value: {err}")))
}

/**
 * Escape the five markup-significant characters so output is safe in both
 * element text and quoted attribute values. HTML and XML differ only in the
 * entity used for the apostrophe (&apos; is not defined in HTML 4).
 * @author: skitsanos
 */
fn escape_markup(input: &str, apostrophe: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str(apostrophe),
            _ => escaped.push(ch),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::{filter_html_escape, filter_html_unescape, filter_xml_escape};
    use serde_json::Value;
    use std::collections::HashMap;

    fn apply(
        filter: fn(&Value, &HashMap<String, Value>) -> Result<Value, tera::Error>,
        input: &str,
    ) -> String {
        filter(&Value::String(input.to_string()), &HashMap::new())
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn html_escape_covers_attribute_context() {
        assert_eq!(
            apply(filter_html_escape, r#"<a title="x" data-y='z'>&</a>"#),
            "&lt;a title=&quot;x&quot; data-y=&#x27;z&#x27;&gt;&amp;&lt;/a&gt;"
        );
    }

    #[test]
    fn xml_escape_uses_xml_apostrophe_entity() {
        assert_eq!(
            apply(filter_xml_escape, r#"Tom & "Jerry's" <tag>"#),
            "Tom &amp; &quot;Jerry&apos;s&quot; &lt;tag&gt;"
        );
    }

    #[test]
    fn html_unescape_round_trips_html_escape() {
        let original = r#"<p class="q">Tom & 'Jerry'</p>"#;
        let escaped = apply(filter_html_escape, original);
        assert_eq!(apply(filter_html_unescape, &escaped), original);
    }
}
