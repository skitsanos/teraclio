use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

/**
 * Convert bytes array to string filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_bytes_to_str(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    if let Value::Array(bytes) = value {
        let mut byte_vec: Vec<u8> = Vec::with_capacity(bytes.len());
        for (index, byte) in bytes.iter().enumerate() {
            let value = byte.as_u64().ok_or_else(|| {
                Error::message(format!(
                    "Invalid input: expected number at index {index} for bytes_to_str"
                ))
            })?;

            if value > u8::MAX as u64 {
                return Err(Error::message(format!(
                    "Invalid byte value at index {index}: {value}. Expected 0..=255"
                )));
            }

            byte_vec.push(value as u8);
        }

        let result = String::from_utf8_lossy(&byte_vec).into_owned();
        serde_json::to_value(result)
            .map_err(|err| Error::message(format!("Failed to serialize converted value: {err}")))
    } else {
        Err(Error::message("Invalid input: expected an array of bytes"))
    }
}

/**
 * Convert string to bytes array filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_str_to_bytes(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    if let Value::String(string) = value {
        let bytes: Vec<Value> = string
            .bytes()
            .map(|b| Value::Number(serde_json::Number::from(b)))
            .collect();
        serde_json::to_value(bytes)
            .map_err(|err| Error::message(format!("Failed to serialize converted value: {err}")))
    } else {
        Err(Error::message("Invalid input: expected a string"))
    }
}

#[cfg(test)]
mod tests {
    use super::filter_bytes_to_str;
    use serde_json::json;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn bytes_to_str_rejects_invalid_item_type() {
        let input = Value::Array(vec![json!("not-byte"), json!(1)]);
        let result = filter_bytes_to_str(&input, &HashMap::new());
        assert!(result.is_err());
    }

    #[test]
    fn bytes_to_str_rejects_out_of_range_byte() {
        let input = Value::Array(vec![json!(500)]);
        let result = filter_bytes_to_str(&input, &HashMap::new());
        assert!(result.is_err());
    }
}
