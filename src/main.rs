use crate::cli::{Cli, generate_completions};
use crate::engine::TemplateEngine;
use crate::error::{Result, TeraclioError};
use crate::utils::{InputFormat, parse_data_source};
use clap::Parser;
use notify::{RecursiveMode, Watcher, recommended_watcher};
use serde_json::Value;
use std::ffi::OsString;
use std::path::Path;
use std::sync::mpsc;

mod cli;
mod engine;
mod error;
mod filters;
mod utils;

const AVAILABLE_FILTERS: &[(&str, &str)] = &[
    // Hash & Security
    ("md5", "Generate MD5 hash"),
    ("sha1", "Generate SHA-1 hash"),
    ("sha256", "Generate SHA-256 hash"),
    ("hmac_sha256", "HMAC-SHA256 signing (arg: key)"),
    // Encoding
    ("base64_encode", "Encode to base64"),
    ("base64_decode", "Decode from base64"),
    ("url_encode", "URL-encode a string"),
    ("url_decode", "URL-decode a string"),
    // Escape
    ("html_escape", "Escape HTML entities"),
    ("html_unescape", "Unescape HTML entities"),
    ("xml_escape", "Escape XML special characters"),
    // Serialization
    ("json_encode", "Serialize value to pretty JSON"),
    ("yaml_encode", "Serialize value to YAML"),
    // Text
    ("truncate_words", "Truncate to N words (args: count, end)"),
    ("regex_replace", "Regex find-and-replace (args: pattern, replacement)"),
    // Case conversion
    ("snake_case", "Convert to snake_case"),
    ("kebab_case", "Convert to kebab-case"),
    ("camel_case", "Convert to camelCase"),
    ("pascal_case", "Convert to PascalCase"),
    ("slug", "Convert to URL-friendly slug"),
    // Date
    ("date_format", "Parse and reformat dates (arg: format)"),
    // UUID
    ("uuid", "Generate a UUID v4"),
    // Bytes
    ("bytes_to_str", "Convert byte array to string"),
    ("str_to_bytes", "Convert string to byte array"),
];

/**
 * Teraclio - CLI tool for template rendering with Tera
 * @author: skitsanos
 */
fn main() -> Result<()> {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
    Ok(())
}

/**
 * Deep-merge two JSON values. If both are objects, recursively merge keys.
 * Otherwise the overlay value wins.
 * @author: skitsanos
 */
fn merge_json(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            for (key, value) in overlay_map {
                let entry = base_map.entry(key).or_insert(Value::Null);
                merge_json(entry, value);
            }
        }
        (base, overlay) => {
            *base = overlay;
        }
    }
}

/**
 * Parse and merge all data sources, inject env vars and --set variables
 * @author: skitsanos
 */
fn parse_data(args: &Cli) -> Result<Value> {
    // Validate that stdin source '-' appears at most once and is last
    let stdin_positions: Vec<usize> = args
        .json_source
        .iter()
        .enumerate()
        .filter(|(_, s)| s.as_str() == "-")
        .map(|(i, _)| i)
        .collect();

    if stdin_positions.len() > 1 {
        return Err(TeraclioError::InvalidInput(
            "Stdin source '-' can only be specified once.".to_string(),
        ));
    }

    if let Some(&pos) = stdin_positions.first() {
        if pos != args.json_source.len() - 1 {
            return Err(TeraclioError::InvalidInput(
                "Stdin source '-' must be the last source specified.".to_string(),
            ));
        }
    }

    // Parse and deep-merge all data sources left to right
    let mut json_data = Value::Object(serde_json::Map::new());
    for source in &args.json_source {
        let data = parse_data_source(source, args.input_format)?;
        merge_json(&mut json_data, data);
    }

    // Add environment variables if requested
    if args.include_env_vars {
        if let Value::Object(ref mut map) = json_data {
            let env_vars: serde_json::Map<String, Value> = std::env::vars()
                .map(|(k, v)| (k, Value::String(v)))
                .collect();
            map.insert("env".to_string(), Value::Object(env_vars));
        } else {
            return Err(TeraclioError::InvalidInput(
                "Cannot include environment variables: data source must be a JSON object when --env-vars is used."
                    .to_string(),
            ));
        }
    }

    // Inject ad-hoc variables from --set key=value options
    for entry in &args.set_vars {
        if let Some(pos) = entry.find('=') {
            let key = &entry[..pos];
            let value = &entry[pos + 1..];
            if let Value::Object(ref mut map) = json_data {
                map.insert(key.to_string(), Value::String(value.to_string()));
            } else {
                return Err(TeraclioError::InvalidInput(
                    "Cannot set variables: data source must be a JSON object when --set is used."
                        .to_string(),
                ));
            }
        } else {
            return Err(TeraclioError::InvalidInput(format!(
                "Invalid --set format '{}': expected KEY=VALUE",
                entry
            )));
        }
    }

    Ok(json_data)
}

/**
 * Validate that rendered output is well-formed in the specified format
 * @author: skitsanos
 */
fn validate_output(content: &str, format: InputFormat) -> Result<()> {
    match format {
        InputFormat::Json => {
            serde_json::from_str::<serde_json::Value>(content)
                .map_err(|e| TeraclioError::InvalidInput(format!("Output is not valid JSON: {e}")))?;
        }
        InputFormat::Yaml => {
            serde_yaml::from_str::<serde_json::Value>(content)
                .map_err(|e| TeraclioError::InvalidInput(format!("Output is not valid YAML: {e}")))?;
        }
        InputFormat::Toml => {
            toml::from_str::<toml::Value>(content)
                .map_err(|e| TeraclioError::InvalidInput(format!("Output is not valid TOML: {e}")))?;
        }
    }
    Ok(())
}

/**
 * Show a unified diff between the rendered output and the existing destination file
 * @author: skitsanos
 */
fn show_diff(rendered: &str, dest_path: &Path, quiet: bool) -> Result<()> {
    let existing = if dest_path.exists() {
        std::fs::read_to_string(dest_path)?
    } else {
        String::new()
    };

    let diff = similar::TextDiff::from_lines(existing.as_str(), rendered);
    let unified = diff
        .unified_diff()
        .header(
            &dest_path.display().to_string(),
            &dest_path.display().to_string(),
        )
        .to_string();

    if unified.is_empty() {
        if !quiet {
            eprintln!("[teraclio] No differences found.");
        }
    } else {
        print!("{unified}");
    }

    Ok(())
}

/**
 * Render template once with the given arguments
 * @author: skitsanos
 */
fn render_once(args: &Cli) -> Result<()> {
    let json_data = parse_data(args)?;
    let template_path = require_template_path(args)?;

    let mut engine = TemplateEngine::new(args.strict);
    engine.load_template(template_path)?;

    let rendered = engine.render(template_path, &json_data)?;

    // Validate output format if specified
    if let Some(format) = args.output_format {
        validate_output(&rendered, format)?;
    }

    // Show diff instead of writing if --diff is set
    if args.diff {
        let dest_path = args.output_file.as_ref().ok_or_else(|| {
            TeraclioError::InvalidInput("--dest is required when using --diff".to_string())
        })?;
        return show_diff(&rendered, Path::new(dest_path), args.quiet);
    }

    let output_path = args
        .output_file
        .as_ref()
        .map(|p| p.as_ref() as &std::path::Path);
    TemplateEngine::write_output(&rendered, output_path)?;

    Ok(())
}

/**
 * Process a directory of templates in one pass
 * @author: skitsanos
 */
fn run_directory_mode(template_dir: &Path, args: &Cli, json_data: &Value) -> Result<()> {
    let dest_dir = match &args.output_file {
        Some(dest) => Path::new(dest),
        None => {
            return Err(TeraclioError::InvalidInput(
                "--dest is required when using directory mode".to_string(),
            ));
        }
    };

    process_directory(template_dir, dest_dir, json_data, args, args.recursive)
}

/**
 * Recursively (or non-recursively) process a template directory, mirroring
 * the directory structure in the destination.
 * @author: skitsanos
 */
fn process_directory(
    template_dir: &Path,
    dest_dir: &Path,
    json_data: &Value,
    args: &Cli,
    recursive: bool,
) -> Result<()> {
    if !dest_dir.exists() {
        std::fs::create_dir_all(dest_dir)?;
    }

    for entry in std::fs::read_dir(template_dir)? {
        let entry = entry?;
        let path = entry.path();

        let file_name = match entry.file_name().to_str() {
            Some(name) if name.starts_with('.') => continue,
            Some(name) => name.to_string(),
            None => continue,
        };

        if path.is_dir() {
            if recursive {
                process_directory(&path, &dest_dir.join(&file_name), json_data, args, recursive)?;
            }
            continue;
        }

        let mut engine = TemplateEngine::new(args.strict);
        engine.load_template(&path)?;

        let rendered = engine.render(&path, json_data)?;

        let output_path = dest_dir.join(&file_name);
        TemplateEngine::write_output(&rendered, Some(&output_path))?;

        info(args, &format!("[teraclio] Rendered: {file_name}"));
    }

    Ok(())
}

/**
 * Get the required template path from CLI args
 * @author: skitsanos
 */
fn require_template_path(args: &Cli) -> Result<&OsString> {
    args.template_path.as_ref().ok_or_else(|| {
        TeraclioError::InvalidInput("--template is required".to_string())
    })
}

/**
 * Print a message to stderr unless --quiet is set
 * @author: skitsanos
 */
fn info(args: &Cli, msg: &str) {
    if !args.quiet {
        eprintln!("{msg}");
    }
}

/**
 * Main application logic with proper error handling
 * @author: skitsanos
 */
fn run() -> Result<()> {
    let args = Cli::parse();

    // Generate shell completions and exit
    if let Some(shell) = args.completions {
        generate_completions(shell);
        return Ok(());
    }

    // List all available filters and exit
    if args.list_filters {
        println!("Available filters ({}):\n", AVAILABLE_FILTERS.len());
        let max_name = AVAILABLE_FILTERS.iter().map(|(n, _)| n.len()).max().unwrap_or(0);
        for (name, description) in AVAILABLE_FILTERS {
            println!("  {name:<max_name$}  {description}");
        }
        return Ok(());
    }

    let template_path_os = require_template_path(&args)?;
    let template_path = Path::new(template_path_os);

    // Directory mode: process all files in the template directory
    if template_path.is_dir() {
        let json_data = parse_data(&args)?;
        return run_directory_mode(template_path, &args, &json_data);
    }

    // Check mode: validate template and data without rendering
    if args.check {
        let _json_data = parse_data(&args)?;
        let mut engine = TemplateEngine::new(args.strict);
        engine.load_template(template_path_os)?;
        info(&args, "Template is valid.");
        return Ok(());
    }

    // Perform the initial render
    render_once(&args)?;

    // If watch mode is enabled, enter the watch loop
    if args.watch {
        for source in &args.json_source {
            if source == "-" {
                return Err(TeraclioError::InvalidInput(
                    "Watch mode cannot be used with stdin ('-') as source.".to_string(),
                ));
            }
        }

        let template_str = template_path_os.to_str().ok_or_else(|| {
            TeraclioError::InvalidInput("Template path is not valid UTF-8.".to_string())
        })?;

        info(&args, "[teraclio] Watching for changes...");

        let (tx, rx) = mpsc::channel();
        let mut watcher = recommended_watcher(tx)?;
        watcher.watch(Path::new(template_str), RecursiveMode::NonRecursive)?;
        for source in &args.json_source {
            watcher.watch(Path::new(source.as_str()), RecursiveMode::NonRecursive)?;
        }

        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    if event.kind.is_modify() {
                        info(&args, "[teraclio] Detected change, re-rendering...");
                        if let Err(e) = render_once(&args) {
                            eprintln!("[teraclio] Re-render error: {e}");
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("[teraclio] Watch error: {e}");
                }
                Err(e) => {
                    return Err(TeraclioError::WatchError(format!(
                        "Watch channel closed: {e}"
                    )));
                }
            }
        }
    }

    Ok(())
}
