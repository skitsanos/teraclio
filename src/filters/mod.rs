/**
 * Template filters module for Tera template engine
 * @author: skitsanos
 */
pub mod base64;
pub mod bytes;
pub mod case;
pub mod date;
pub mod escape;
pub mod hash;
pub mod regex;
pub mod serialize;
pub mod text;
pub mod url;
pub mod uuid;

use serde_json::Value;
use std::collections::HashMap;

/// Signature shared by every Teraclio filter
pub type FilterFn = fn(&Value, &HashMap<String, Value>) -> Result<Value, tera::Error>;

/// One custom filter: template name, `--list-filters` description, implementation.
pub struct FilterSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub filter: FilterFn,
}

const fn spec(name: &'static str, description: &'static str, filter: FilterFn) -> FilterSpec {
    FilterSpec {
        name,
        description,
        filter,
    }
}

/// Single source of truth: the engine registers exactly these, and
/// `--list-filters` prints exactly these, so the two cannot drift apart.
pub const FILTERS: &[FilterSpec] = &[
    // Hash & Security
    spec("md5", "Generate MD5 hash", hash::filter_md5),
    spec("sha1", "Generate SHA-1 hash", hash::filter_sha1),
    spec("sha256", "Generate SHA-256 hash", hash::filter_sha256),
    spec(
        "hmac_sha256",
        "HMAC-SHA256 signing (arg: key)",
        hash::filter_hmac_sha256,
    ),
    // Encoding
    spec(
        "base64_encode",
        "Encode to base64",
        base64::filter_base64_encode,
    ),
    spec(
        "base64_decode",
        "Decode from base64 (text, or a byte array if not UTF-8)",
        base64::filter_base64_decode,
    ),
    spec("url_encode", "URL-encode a string", url::filter_url_encode),
    spec("url_decode", "URL-decode a string", url::filter_url_decode),
    // Escape
    spec(
        "html_escape",
        "Escape HTML entities",
        escape::filter_html_escape,
    ),
    spec(
        "html_unescape",
        "Unescape HTML entities",
        escape::filter_html_unescape,
    ),
    spec(
        "xml_escape",
        "Escape XML special characters",
        escape::filter_xml_escape,
    ),
    // Serialization
    spec(
        "json_encode",
        "Serialize value to pretty JSON",
        serialize::filter_json_encode,
    ),
    spec(
        "yaml_encode",
        "Serialize value to YAML",
        serialize::filter_yaml_encode,
    ),
    // Text
    spec(
        "truncate_words",
        "Truncate to N words (args: count, end)",
        text::filter_truncate_words,
    ),
    spec(
        "regex_replace",
        "Regex find-and-replace (args: pattern, replacement)",
        regex::filter_regex_replace,
    ),
    // Case conversion
    spec(
        "snake_case",
        "Convert to snake_case",
        case::filter_snake_case,
    ),
    spec(
        "kebab_case",
        "Convert to kebab-case",
        case::filter_kebab_case,
    ),
    spec(
        "camel_case",
        "Convert to camelCase",
        case::filter_camel_case,
    ),
    spec(
        "pascal_case",
        "Convert to PascalCase",
        case::filter_pascal_case,
    ),
    spec("slug", "Convert to URL-friendly slug", case::filter_slug),
    // Date
    spec(
        "date_format",
        "Parse and reformat dates (arg: format)",
        date::filter_date_format,
    ),
    // UUID
    spec("uuid", "Generate a UUID v4", uuid::filter_uuid),
    // Bytes
    spec(
        "bytes_to_str",
        "Convert byte array to string",
        bytes::filter_bytes_to_str,
    ),
    spec(
        "str_to_bytes",
        "Convert string to byte array",
        bytes::filter_str_to_bytes,
    ),
];
