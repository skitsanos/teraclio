use crate::error::{Result, TeraclioError};
use crate::filters::base64::{filter_base64_decode, filter_base64_encode};
use crate::filters::bytes::{filter_bytes_to_str, filter_str_to_bytes};
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
}

impl TemplateEngine {
    /**
     * Create a new TemplateEngine instance with registered filters
     * @author: skitsanos
     */
    pub fn new() -> Self {
        let mut tera = Tera::default();

        tera.register_filter("base64_encode", filter_base64_encode);
        tera.register_filter("base64_decode", filter_base64_decode);
        tera.register_filter("bytes_to_str", filter_bytes_to_str);
        tera.register_filter("str_to_bytes", filter_str_to_bytes);

        Self { tera }
    }

    /**
     * Load a template file into the engine
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
