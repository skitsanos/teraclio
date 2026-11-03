use crate::error::{Result, TeraclioError};
use crate::filters::base64::{filter_base64_decode, filter_base64_encode};
use crate::filters::bytes::{filter_bytes_to_str, filter_str_to_bytes};
use crate::filters::case::{
    filter_camel_case, filter_kebab_case, filter_pascal_case, filter_slug, filter_snake_case,
};
use crate::filters::date::filter_date_format;
use crate::filters::escape::{filter_html_escape, filter_html_unescape, filter_xml_escape};
use crate::filters::hash::{filter_hmac_sha256, filter_md5, filter_sha1, filter_sha256};
use crate::filters::regex::filter_regex_replace;
use crate::filters::serialize::{filter_json_encode, filter_yaml_encode};
use crate::filters::text::filter_truncate_words;
use crate::filters::url::{filter_url_decode, filter_url_encode};
use crate::filters::uuid::filter_uuid;
use serde_json::Value as JsonValue;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tera::{Context, Kwargs, State, Tera, Value};

type LegacyFilter = fn(
    &JsonValue,
    &std::collections::HashMap<String, JsonValue>,
) -> std::result::Result<JsonValue, tera::Error>;

fn adapt_filter(
    filter: LegacyFilter,
) -> impl Fn(&Value, Kwargs, &State) -> tera::TeraResult<Value> {
    move |value, kwargs, _state| {
        let input = serde_json::to_value(value).map_err(|err| {
            tera::Error::message(format!("Failed to convert filter input: {err}"))
        })?;
        let args = kwargs.deserialize::<std::collections::HashMap<String, JsonValue>>()?;
        let output = filter(&input, &args)?;
        Value::try_from_serializable(&output)
    }
}

/**
 * Template engine for processing Tera templates with custom filters
 * @author: skitsanos
 */
pub struct TemplateEngine {
    tera: Tera,
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEngine {
    /**
     * Create a new TemplateEngine instance with registered filters
     * @author: skitsanos
     */
    pub fn new() -> Self {
        let mut tera = Tera::default();

        // Base64 filters
        tera.register_filter("base64_encode", adapt_filter(filter_base64_encode));
        tera.register_filter("base64_decode", adapt_filter(filter_base64_decode));

        // Bytes conversion filters
        tera.register_filter("bytes_to_str", adapt_filter(filter_bytes_to_str));
        tera.register_filter("str_to_bytes", adapt_filter(filter_str_to_bytes));

        // Hash filters
        tera.register_filter("md5", adapt_filter(filter_md5));
        tera.register_filter("sha1", adapt_filter(filter_sha1));
        tera.register_filter("sha256", adapt_filter(filter_sha256));
        tera.register_filter("hmac_sha256", adapt_filter(filter_hmac_sha256));

        // URL filters
        tera.register_filter("url_encode", adapt_filter(filter_url_encode));
        tera.register_filter("url_decode", adapt_filter(filter_url_decode));

        // HTML/XML escape filters
        tera.register_filter("html_escape", adapt_filter(filter_html_escape));
        tera.register_filter("html_unescape", adapt_filter(filter_html_unescape));
        tera.register_filter("xml_escape", adapt_filter(filter_xml_escape));

        // Serialization filters
        tera.register_filter("json_encode", adapt_filter(filter_json_encode));
        tera.register_filter("yaml_encode", adapt_filter(filter_yaml_encode));

        // Regex filters
        tera.register_filter("regex_replace", adapt_filter(filter_regex_replace));

        // Text filters
        tera.register_filter("truncate_words", adapt_filter(filter_truncate_words));

        // Date filters
        tera.register_filter("date_format", adapt_filter(filter_date_format));

        // UUID filter
        tera.register_filter("uuid", adapt_filter(filter_uuid));

        // Case conversion filters
        tera.register_filter("snake_case", adapt_filter(filter_snake_case));
        tera.register_filter("kebab_case", adapt_filter(filter_kebab_case));
        tera.register_filter("camel_case", adapt_filter(filter_camel_case));
        tera.register_filter("pascal_case", adapt_filter(filter_pascal_case));
        tera.register_filter("slug", adapt_filter(filter_slug));

        Self { tera }
    }

    /**
     * Load a template file into the engine, registering sibling files in the
     * same directory under their bare file names so that
     * {% include "sibling.html" %} and {% extends "base.html" %} resolve.
     * Paths listed in `exclude` (data sources, the output file) are never
     * parsed as templates.
     * @author: skitsanos
     */
    pub fn load_template<P: AsRef<Path>>(
        &mut self,
        template_path: P,
        exclude: &[PathBuf],
    ) -> Result<()> {
        let path = template_path.as_ref();

        if !path.exists() {
            return Err(TeraclioError::InvalidInput(format!(
                "Template file does not exist: {}",
                path.display()
            )));
        }

        // A bare file name has an empty parent; its siblings live in ".".
        let parent = match path.parent() {
            Some(dir) if !dir.as_os_str().is_empty() => dir,
            _ => Path::new("."),
        };
        let own_path = absolute_path(path);
        let excluded: HashSet<PathBuf> = exclude.iter().map(|p| absolute_path(p)).collect();

        let mut siblings = Vec::new();
        if let Ok(entries) = std::fs::read_dir(parent) {
            for entry in entries.flatten() {
                let sibling_path = entry.path();
                if !sibling_path.is_file() {
                    continue;
                }
                let file_name = match sibling_path.file_name().and_then(|n| n.to_str()) {
                    Some(name) if !name.starts_with('.') => name.to_string(),
                    _ => continue,
                };
                let sibling_abs = absolute_path(&sibling_path);
                if sibling_abs == own_path || excluded.contains(&sibling_abs) {
                    continue;
                }
                siblings.push((sibling_path, file_name));
            }
        }
        self.register_siblings(siblings);

        self.tera.add_template_file(path, None)?;
        Ok(())
    }

    /**
     * Register sibling templates regardless of directory iteration order.
     * A single batch covers the common case. If any file fails (unparseable,
     * or a child whose parent is not loaded yet), fall back to repeated
     * passes so parents added in one pass satisfy children in the next.
     * Files that never load are skipped; Tera reports them as unknown
     * templates only if something actually references them.
     * @author: skitsanos
     */
    fn register_siblings(&mut self, siblings: Vec<(PathBuf, String)>) {
        if siblings.is_empty()
            || self
                .tera
                .add_template_files(siblings.iter().map(|(path, name)| (path, Some(name))))
                .is_ok()
        {
            return;
        }

        let mut pending = siblings;
        loop {
            let before = pending.len();
            pending.retain(|(path, name)| self.tera.add_template_file(path, Some(name)).is_err());
            if pending.is_empty() || pending.len() == before {
                break;
            }
        }
    }

    /**
     * Render a template with the provided JSON data
     * @author: skitsanos
     */
    pub fn render<P: AsRef<Path>>(
        &self,
        template_path: P,
        json_data: &JsonValue,
    ) -> Result<String> {
        let template_name = template_path.as_ref().to_string_lossy();

        let mut context = Context::new();
        context.insert("data", json_data);

        let rendered = self.tera.render(&template_name, &context)?;
        Ok(rendered)
    }

    /**
     * Write rendered content to a file or stdout
     * @author: skitsanos
     */
    pub fn write_output<P: AsRef<Path>>(content: &str, output_path: Option<P>) -> Result<()> {
        match output_path {
            Some(path) => {
                let mut file = File::create(path)?;
                file.write_all(content.as_bytes())?;
                file.flush()?;
            }
            None => {
                io::stdout().write_all(content.as_bytes())?;
                io::stdout().flush()?;
            }
        }
        Ok(())
    }
}

/**
 * Absolute form of a path for identity comparisons, without touching the
 * filesystem (files that are about to be created still compare correctly).
 * @author: skitsanos
 */
pub fn absolute_path(path: &Path) -> PathBuf {
    std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf())
}
