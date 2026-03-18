use crate::error::{Result, TeraclioError};
use crate::filters::base64::{filter_base64_decode, filter_base64_encode};
use crate::filters::bytes::{filter_bytes_to_str, filter_str_to_bytes};
use crate::filters::case::{
    filter_camel_case, filter_kebab_case, filter_pascal_case, filter_slug, filter_snake_case,
};
use crate::filters::escape::{filter_html_escape, filter_html_unescape, filter_xml_escape};
use crate::filters::date::filter_date_format;
use crate::filters::hash::{filter_hmac_sha256, filter_md5, filter_sha1, filter_sha256};
use crate::filters::regex::filter_regex_replace;
use crate::filters::serialize::{filter_json_encode, filter_yaml_encode};
use crate::filters::text::filter_truncate_words;
use crate::filters::url::{filter_url_decode, filter_url_encode};
use crate::filters::uuid::filter_uuid;
use serde_json::Value;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use tera::{Context, Tera};

/**
 * Template engine for processing Tera templates with custom filters
 * @author: skitsanos
 */
pub struct TemplateEngine {
    tera: Tera,
    #[allow(dead_code)]
    strict: bool,
}

impl TemplateEngine {
    /**
     * Create a new TemplateEngine instance with registered filters
     * @author: skitsanos
     */
    pub fn new(strict: bool) -> Self {
        let mut tera = Tera::default();

        // Base64 filters
        tera.register_filter("base64_encode", filter_base64_encode);
        tera.register_filter("base64_decode", filter_base64_decode);

        // Bytes conversion filters
        tera.register_filter("bytes_to_str", filter_bytes_to_str);
        tera.register_filter("str_to_bytes", filter_str_to_bytes);

        // Hash filters
        tera.register_filter("md5", filter_md5);
        tera.register_filter("sha1", filter_sha1);
        tera.register_filter("sha256", filter_sha256);
        tera.register_filter("hmac_sha256", filter_hmac_sha256);

        // URL filters
        tera.register_filter("url_encode", filter_url_encode);
        tera.register_filter("url_decode", filter_url_decode);

        // HTML/XML escape filters
        tera.register_filter("html_escape", filter_html_escape);
        tera.register_filter("html_unescape", filter_html_unescape);
        tera.register_filter("xml_escape", filter_xml_escape);

        // Serialization filters
        tera.register_filter("json_encode", filter_json_encode);
        tera.register_filter("yaml_encode", filter_yaml_encode);

        // Regex filters
        tera.register_filter("regex_replace", filter_regex_replace);

        // Text filters
        tera.register_filter("truncate_words", filter_truncate_words);

        // Date filters
        tera.register_filter("date_format", filter_date_format);

        // UUID filter
        tera.register_filter("uuid", filter_uuid);

        // Case conversion filters
        tera.register_filter("snake_case", filter_snake_case);
        tera.register_filter("kebab_case", filter_kebab_case);
        tera.register_filter("camel_case", filter_camel_case);
        tera.register_filter("pascal_case", filter_pascal_case);
        tera.register_filter("slug", filter_slug);

        Self { tera, strict }
    }

    /**
     * Load a template file into the engine, registering sibling files
     * in the same directory so that {% include "sibling.html" %} works.
     * @author: skitsanos
     */
    pub fn load_template<P: AsRef<Path>>(&mut self, template_path: P) -> Result<()> {
        let path = template_path.as_ref();

        if !path.exists() {
            return Err(TeraclioError::InvalidInput(format!(
                "Template file does not exist: {}",
                path.display()
            )));
        }

        // Register sibling files so Tera's {% include %} directive can find them
        if let Some(parent) = path.parent() {
            if parent.exists() {
                if let Ok(entries) = std::fs::read_dir(parent) {
                    for entry in entries.flatten() {
                        let sibling_path = entry.path();

                        if sibling_path.is_dir() {
                            continue;
                        }

                        if sibling_path == path {
                            continue;
                        }

                        let file_name = match sibling_path.file_name().and_then(|n| n.to_str()) {
                            Some(name) => name.to_string(),
                            None => continue,
                        };
                        if file_name.starts_with('.') {
                            continue;
                        }

                        // Silently skip files that can't be parsed as templates
                        let _ =
                            self.tera.add_template_file(&sibling_path, Some(&file_name));
                    }
                }
            }
        }

        self.tera.add_template_file(path, None)?;
        Ok(())
    }

    /**
     * Render a template with the provided JSON data
     * @author: skitsanos
     */
    pub fn render<P: AsRef<Path>>(&self, template_path: P, json_data: &Value) -> Result<String> {
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
