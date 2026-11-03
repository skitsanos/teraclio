use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * Base64 encode filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_base64_encode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::message("Invalid input: expected a string for base64_encode"))?;
    let encoded_value = general_purpose::STANDARD.encode(input_str);
    serde_json::to_value(encoded_value)
        .map_err(|err| Error::message(format!("Failed to serialize encoded value: {err}")))
}

/**
 * Base64 decode filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_base64_decode(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::message("Invalid input: expected a string for base64_decode"))?;

    let decoded = general_purpose::STANDARD
        .decode(input_str)
        .map_err(|_| Error::message("Failed to decode Base64: invalid input"))?;

    // Text is the common case, so return it directly. Binary payloads that
    // are not valid UTF-8 stay a byte array rather than being corrupted.
    let value = match String::from_utf8(decoded) {
        Ok(text) => Value::String(text),
        Err(err) => serde_json::to_value(err.into_bytes())
            .map_err(|err| Error::message(format!("Failed to serialize decoded value: {err}")))?,
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{filter_base64_decode, filter_base64_encode};
    use serde_json::{json, Value};
    use std::collections::HashMap;

    fn decode(input: &str) -> Value {
        filter_base64_decode(&Value::String(input.to_string()), &HashMap::new()).unwrap()
    }

    #[test]
    fn decodes_text_to_a_string() {
        assert_eq!(decode("SGVsbG8g8J+Mjg=="), json!("Hello 🌎"));
    }

    #[test]
    fn non_utf8_payload_stays_bytes() {
        assert_eq!(decode("/w=="), json!([255]));
    }

    #[test]
    fn round_trips_with_encode() {
        let encoded =
            filter_base64_encode(&Value::String("teraclio".into()), &HashMap::new()).unwrap();
        assert_eq!(decode(encoded.as_str().unwrap()), json!("teraclio"));
    }

    #[test]
    fn invalid_input_is_an_error() {
        assert!(filter_base64_decode(&json!("not base64!"), &HashMap::new()).is_err());
    }
}
