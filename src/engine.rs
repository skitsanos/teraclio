use crate::error::{Result, TeraclioError};
use crate::filters::{FilterFn, FILTERS};
use crate::functions;
use serde_json::Value as JsonValue;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tera::{Context, Kwargs, State, Tera, Value};

fn adapt_filter(filter: FilterFn) -> impl Fn(&Value, Kwargs, &State) -> tera::TeraResult<Value> {
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
        for spec in FILTERS {
            tera.register_filter(spec.name, adapt_filter(spec.filter));
        }
        tera.register_function("now", functions::now);
        tera.autoescape_on(autoescape_suffixes());

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
        let mut skip: HashSet<PathBuf> = exclude.iter().map(|p| absolute_path(p)).collect();
        skip.insert(absolute_path(path));
        self.register_all(template_files(parent, &skip));

        self.tera.add_template_file(path, None)?;
        Ok(())
    }

    /**
     * Register templates regardless of directory iteration order.
     * A single batch covers the common case. If any file fails (unparseable,
     * or a child whose parent is not loaded yet), fall back to repeated
     * passes so parents added in one pass satisfy children in the next.
     * Files that never load are skipped; Tera reports them as unknown
     * templates only if something actually references them.
     * @author: skitsanos
     */
    fn register_all(&mut self, siblings: Vec<(PathBuf, String)>) {
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
     * Register every template file in `dir` under its bare file name, once,
     * so a directory of N templates is parsed N times instead of N².
     * Returns the names that loaded; anything else failed to parse.
     * @author: skitsanos
     */
    pub fn load_directory(&mut self, dir: &Path, exclude: &[PathBuf]) -> HashSet<String> {
        let skip: HashSet<PathBuf> = exclude.iter().map(|p| absolute_path(p)).collect();
        let files = template_files(dir, &skip);
        let names: Vec<String> = files.iter().map(|(_, name)| name.clone()).collect();
        self.register_all(files);
        let loaded: HashSet<&str> = self.tera.get_template_names().collect();
        names
            .into_iter()
            .filter(|name| loaded.contains(name.as_str()))
            .collect()
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
        self.render_named(&template_path.as_ref().to_string_lossy(), json_data)
    }

    /**
     * Render an already registered template by name
     * @author: skitsanos
     */
    pub fn render_named(&self, name: &str, json_data: &JsonValue) -> Result<String> {
        let mut context = Context::new();
        context.insert("data", json_data);
        Ok(self.tera.render(name, &context)?)
    }

    /**
     * Write rendered content to a file or stdout
     * @author: skitsanos
     */
    pub fn write_output<P: AsRef<Path>>(content: &str, output_path: Option<P>) -> Result<()> {
        match output_path {
            Some(path) => write_atomically(path.as_ref(), content),
            None => {
                let mut stdout = io::stdout().lock();
                stdout.write_all(content.as_bytes())?;
                stdout.flush()?;
                Ok(())
            }
        }
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

/// File suffixes that only mark a file as a template (`page.html.tera`).
/// Directory mode strips them from output names.
pub const TEMPLATE_SUFFIXES: &[&str] = &[".tera", ".j2", ".jinja", ".jinja2"];

/// Formats Tera auto-escapes by default.
const ESCAPED_FORMATS: &[&str] = &[".html", ".htm", ".xml"];

/**
 * Tera picks autoescaping by the template name's suffix, so `page.html.tera`
 * would lose the escaping that `page.html` gets. Escape both forms.
 * @author: skitsanos
 */
fn autoescape_suffixes() -> Vec<String> {
    ESCAPED_FORMATS
        .iter()
        .flat_map(|format| {
            std::iter::once(format.to_string()).chain(
                TEMPLATE_SUFFIXES
                    .iter()
                    .map(move |suffix| format!("{format}{suffix}")),
            )
        })
        .collect()
}

/**
 * Output file name for a template in directory mode: `config.yaml.tera` ->
 * `config.yaml`. Names that would become empty are kept as they are.
 * @author: skitsanos
 */
pub fn output_name(template_name: &str) -> &str {
    TEMPLATE_SUFFIXES
        .iter()
        .find_map(|suffix| template_name.strip_suffix(suffix))
        .filter(|stem| !stem.is_empty())
        .unwrap_or(template_name)
}

/**
 * Non-hidden regular files in `dir` as (path, bare name) pairs, minus `skip`
 * (absolute paths). Sorted by name for deterministic behavior.
 * @author: skitsanos
 */
fn template_files(dir: &Path, skip: &HashSet<PathBuf>) -> Vec<(PathBuf, String)> {
    let mut files: Vec<(PathBuf, String)> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let name = entry.file_name().to_str()?.to_string();
            let eligible =
                !name.starts_with('.') && path.is_file() && !skip.contains(&absolute_path(&path));
            eligible.then_some((path, name))
        })
        .collect();
    files.sort_by(|a, b| a.1.cmp(&b.1));
    files
}

/**
 * Replace `path` atomically: write a hidden temp file next to it, then rename
 * it over the original. Readers (dev servers, watchers) never see a truncated
 * file. Missing parent directories are created, symlinks are written through
 * to their target, and an existing file's permissions are kept.
 * @author: skitsanos
 */
fn write_atomically(path: &Path, content: &str) -> Result<()> {
    let target = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let dir = match target.parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir.to_path_buf(),
        _ => PathBuf::from("."),
    };
    let file_name = target.file_name().ok_or_else(|| {
        TeraclioError::InvalidInput(format!("Not a file path: {}", path.display()))
    })?;
    std::fs::create_dir_all(&dir)?;

    let temp = dir.join(format!(
        ".{}.teraclio-{}.tmp",
        file_name.to_string_lossy(),
        std::process::id()
    ));
    let result = (|| -> io::Result<()> {
        let mut file = File::create(&temp)?;
        file.write_all(content.as_bytes())?;
        file.flush()?;
        if let Ok(existing) = std::fs::metadata(&target) {
            std::fs::set_permissions(&temp, existing.permissions())?;
        }
        std::fs::rename(&temp, &target)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    Ok(result?)
}

#[cfg(test)]
mod tests {
    use super::{autoescape_suffixes, output_name};

    #[test]
    fn output_name_strips_one_template_suffix() {
        assert_eq!(output_name("config.yaml.tera"), "config.yaml");
        assert_eq!(output_name("page.html.j2"), "page.html");
        assert_eq!(output_name("Dockerfile.jinja2"), "Dockerfile");
        assert_eq!(output_name("plain.txt"), "plain.txt");
        // Nothing left after stripping: keep the name
        assert_eq!(output_name(".tera"), ".tera");
    }

    #[test]
    fn suffixed_html_templates_stay_autoescaped() {
        let suffixes = autoescape_suffixes();
        for name in [".html", ".xml", ".html.tera", ".htm.j2", ".xml.jinja2"] {
            assert!(suffixes.iter().any(|s| s == name), "missing {name}");
        }
    }
}
