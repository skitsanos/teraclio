use hmac::{Hmac, Mac};
use serde_json::Value;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use tera::Error;

type HmacSha256 = Hmac<Sha256>;

/**
 * MD5 hash filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_md5(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for md5"))?;
    let result = md5::compute(input_str.as_bytes());
    let hash_string = format!("{result:x}");
    tera::to_value(hash_string)
        .map_err(|err| Error::msg(format!("Failed to serialize hash value: {err}")))
}

/**
 * SHA1 hash filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_sha1(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for sha1"))?;
    let mut hasher = Sha1::new();
    Digest::update(&mut hasher, input_str.as_bytes());
    let result = hasher.finalize();
    let hash_string = format!("{result:x}");
    tera::to_value(hash_string)
        .map_err(|err| Error::msg(format!("Failed to serialize hash value: {err}")))
}

/**
 * SHA256 hash filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_sha256(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for sha256"))?;
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, input_str.as_bytes());
    let result = hasher.finalize();
    let hash_string = format!("{result:x}");
    tera::to_value(hash_string)
        .map_err(|err| Error::msg(format!("Failed to serialize hash value: {err}")))
}

/**
 * HMAC-SHA256 filter for Tera templates
 * @author: skitsanos
 */
pub fn filter_hmac_sha256(value: &Value, args: &HashMap<String, Value>) -> Result<Value, Error> {
    let input_str = value
        .as_str()
        .ok_or_else(|| Error::msg("Invalid input: expected a string for hmac_sha256"))?;
    let key = args
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("Missing required argument: key"))?;
    let mut mac =
        HmacSha256::new_from_slice(key.as_bytes()).map_err(|err| Error::msg(format!("{err}")))?;
    mac.update(input_str.as_bytes());
    let result = mac.finalize();
    let hash_string = format!("{:x}", result.into_bytes());
    tera::to_value(hash_string)
        .map_err(|err| Error::msg(format!("Failed to serialize hash value: {err}")))
}

#[cfg(test)]
mod tests {
    use super::{filter_hmac_sha256, filter_md5, filter_sha1, filter_sha256};
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn hashes_return_expected_values() {
        let input = Value::String("abc".to_string());
        let args = HashMap::new();
        assert_eq!(
            filter_md5(&input, &args).unwrap(),
            Value::String("900150983cd24fb0d6963f7d28e17f72".to_string())
        );
        assert_eq!(
            filter_sha1(&input, &args).unwrap(),
            Value::String("a9993e364706816aba3e25717850c26c9cd0d89d".to_string())
        );
        assert_eq!(
            filter_sha256(&input, &args).unwrap(),
            Value::String(
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".to_string()
            )
        );
    }

    #[test]
    fn hash_rejects_non_string_input() {
        let args = HashMap::new();
        let input = Value::Number(serde_json::Number::from(1));
        assert!(filter_md5(&input, &args).is_err());
        assert!(filter_sha1(&input, &args).is_err());
        assert!(filter_sha256(&input, &args).is_err());
    }

    #[test]
    fn hmac_sha256_returns_valid_hex() {
        let input = Value::String("abc".to_string());
        let mut args = HashMap::new();
        args.insert("key".to_string(), Value::String("secret".to_string()));
        let result = filter_hmac_sha256(&input, &args).unwrap();
        let hex_str = result.as_str().unwrap();
        assert_eq!(hex_str.len(), 64);
        assert!(hex_str.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn hmac_sha256_missing_key_returns_error() {
        let input = Value::String("abc".to_string());
        let args = HashMap::new();
        assert!(filter_hmac_sha256(&input, &args).is_err());
    }
}
