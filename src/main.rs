use crate::cli::{generate_completions, Cli};
use crate::engine::{absolute_path, output_name, TemplateEngine};
use crate::error::{Result, TeraclioError};
use crate::filters::FILTERS;
use crate::functions::FUNCTIONS;
use crate::utils::{parse_data_source, InputFormat};
use clap::Parser;
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Watcher};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::mpsc;
use std::time::{Duration, Instant};

mod cli;
mod engine;
mod error;
mod filters;
mod functions;
mod utils;

/**
 * Teraclio - CLI tool for template rendering with Tera
 * @author: skitsanos
 */
fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
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

    // Inject ad-hoc variables: --set values are strings, --set-json values are
    // parsed as JSON. --set-json runs second, so it wins on the same key.
    for entry in &args.set_vars {
        let (key, value) = split_assignment(entry, "--set", "KEY=VALUE")?;
        set_data_path(&mut json_data, key, Value::String(value.to_string()))?;
    }
    for entry in &args.set_json_vars {
        let (key, raw) = split_assignment(entry, "--set-json", "KEY=JSON")?;
        let value = serde_json::from_str(raw).map_err(|e| {
            TeraclioError::InvalidInput(format!("Invalid JSON for --set-json {key}: {e}"))
        })?;
        set_data_path(&mut json_data, key, value)?;
    }

    Ok(json_data)
}

/**
 * Split `KEY=VALUE` at the first '='
 * @author: skitsanos
 */
fn split_assignment<'a>(entry: &'a str, flag: &str, shape: &str) -> Result<(&'a str, &'a str)> {
    entry.split_once('=').ok_or_else(|| {
        TeraclioError::InvalidInput(format!("Invalid {flag} format '{entry}': expected {shape}"))
    })
}

/**
 * Set a dotted key path (`app.db.port`) inside the data object, creating
 * intermediate objects as needed. Refuses to replace a non-object on the way,
 * which would silently discard data from the sources.
 * @author: skitsanos
 */
fn set_data_path(data: &mut Value, key: &str, value: Value) -> Result<()> {
    let segments: Vec<&str> = key.split('.').collect();
    if segments.iter().any(|segment| segment.is_empty()) {
        return Err(TeraclioError::InvalidInput(format!(
            "Invalid key '{key}': empty segment"
        )));
    }
    let Value::Object(root) = data else {
        return Err(TeraclioError::InvalidInput(
            "Cannot set variables: data source must be a JSON object when --set or --set-json is used."
                .to_string(),
        ));
    };

    let (last, parents) = segments
        .split_last()
        .expect("split always yields one segment");
    let mut cursor = root;
    for (depth, segment) in parents.iter().enumerate() {
        let entry = cursor
            .entry(segment.to_string())
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
        cursor = match entry {
            Value::Object(map) => map,
            _ => {
                return Err(TeraclioError::InvalidInput(format!(
                    "Cannot set '{key}': '{}' is not an object",
                    segments[..=depth].join(".")
                )));
            }
        };
    }
    cursor.insert(last.to_string(), value);
    Ok(())
}

/**
 * Validate that rendered output is well-formed in the specified format
 * @author: skitsanos
 */
fn validate_output(content: &str, format: InputFormat) -> Result<()> {
    match format {
        InputFormat::Json => {
            serde_json::from_str::<serde_json::Value>(content).map_err(|e| {
                TeraclioError::InvalidInput(format!("Output is not valid JSON: {e}"))
            })?;
        }
        InputFormat::Yaml => {
            // Multi-document streams (`---` separated, e.g. Kubernetes manifests) are valid YAML
            noyalib::compat::serde_yaml::from_str_multi::<serde_json::Value>(content).map_err(
                |e| TeraclioError::InvalidInput(format!("Output is not valid YAML: {e}")),
            )?;
        }
        InputFormat::Toml => {
            toml::from_str::<toml::Table>(content).map_err(|e| {
                TeraclioError::InvalidInput(format!("Output is not valid TOML: {e}"))
            })?;
        }
    }
    Ok(())
}

/**
 * Show a unified diff between the rendered output and the existing destination
 * file. Returns whether they differ (a missing destination always differs).
 * @author: skitsanos
 */
fn show_diff(rendered: &str, dest_path: &Path, quiet: bool) -> Result<bool> {
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
        return Ok(false);
    }

    print!("{unified}");
    Ok(true)
}

/**
 * Validate, then check, diff, or write rendered output. Returns true only when
 * --diff found a difference.
 * @author: skitsanos
 */
fn handle_rendered_output(rendered: &str, output_path: Option<&Path>, args: &Cli) -> Result<bool> {
    if let Some(format) = args.output_format {
        validate_output(rendered, format)?;
    }

    if args.check {
        return Ok(false);
    }

    if args.diff {
        let dest_path = output_path.ok_or_else(|| {
            TeraclioError::InvalidInput("--dest is required when using --diff".to_string())
        })?;
        return show_diff(rendered, dest_path, args.quiet);
    }

    TemplateEngine::write_output(rendered, output_path)?;
    Ok(false)
}

/**
 * Data source files, which share a template's directory often enough that
 * they must never be parsed as templates themselves.
 * @author: skitsanos
 */
fn source_paths(args: &Cli) -> Vec<PathBuf> {
    args.json_source
        .iter()
        .filter(|source| source.as_str() != "-")
        .map(PathBuf::from)
        .collect()
}

/**
 * Load one template (plus its siblings) and render it
 * @author: skitsanos
 */
fn render_file(template_path: &Path, json_data: &Value, exclude: &[PathBuf]) -> Result<String> {
    let mut engine = TemplateEngine::new();
    engine.load_template(template_path, exclude)?;
    engine.render(template_path, json_data)
}

/**
 * Render template once with the given arguments
 * @author: skitsanos
 */
fn render_once(args: &Cli) -> Result<bool> {
    let json_data = parse_data(args)?;
    let template_path = Path::new(require_template_path(args)?);
    let output_path = args.output_file.as_ref().map(Path::new);
    if let Some(output) = output_path {
        if absolute_path(output) == absolute_path(template_path) && !args.check && !args.diff {
            return Err(TeraclioError::InvalidInput(
                "--dest must not be the template file itself".to_string(),
            ));
        }
    }
    let mut exclude = source_paths(args);
    exclude.extend(output_path.map(Path::to_path_buf));
    let rendered = render_file(template_path, &json_data, &exclude)?;
    handle_rendered_output(&rendered, output_path, args)
}

/**
 * Process a directory of templates in one pass
 * @author: skitsanos
 */
fn run_directory_mode(template_dir: &Path, args: &Cli, json_data: &Value) -> Result<bool> {
    let dest_dir = args.output_file.as_ref().map(Path::new);
    if !args.check && dest_dir.is_none() {
        return Err(TeraclioError::InvalidInput(
            "--dest is required when using directory mode".to_string(),
        ));
    }

    let dest_root = dest_dir.map(absolute_path);
    if dest_root.as_deref() == Some(absolute_path(template_dir).as_path()) && !args.check {
        return Err(TeraclioError::InvalidInput(
            "--dest must not be the template directory itself".to_string(),
        ));
    }

    let mut walk = DirectoryWalk {
        args,
        json_data,
        dest_root,
        sources: source_paths(args),
        visited: HashSet::new(),
        differs: false,
    };
    walk.process(template_dir, dest_dir.map(Path::to_path_buf))?;
    Ok(walk.differs)
}

/**
 * State for one directory-mode run.
 * @author: skitsanos
 */
struct DirectoryWalk<'a> {
    args: &'a Cli,
    json_data: &'a Value,
    /// Absolute destination root; never descended into, so a destination
    /// inside the template tree is not rendered again on every level.
    dest_root: Option<PathBuf>,
    /// Data source files, never parsed as templates.
    sources: Vec<PathBuf>,
    /// Canonical directories already processed, which breaks symlink cycles.
    visited: HashSet<PathBuf>,
    /// Whether --diff found any difference.
    differs: bool,
}

impl DirectoryWalk<'_> {
    /**
     * Process a template directory (recursively with --recursive), mirroring
     * the directory structure in the destination.
     * @author: skitsanos
     */
    fn process(&mut self, template_dir: &Path, dest_dir: Option<PathBuf>) -> Result<()> {
        let canonical = template_dir
            .canonicalize()
            .unwrap_or_else(|_| absolute_path(template_dir));
        if !self.visited.insert(canonical) {
            return Ok(());
        }

        if let Some(dest_dir) = &dest_dir {
            if !self.args.check && !self.args.diff && !dest_dir.exists() {
                std::fs::create_dir_all(dest_dir)?;
            }
        }

        let mut entries: Vec<_> =
            std::fs::read_dir(template_dir)?.collect::<std::io::Result<_>>()?;
        // Deterministic order for output messages and error reporting
        entries.sort_by_key(|entry| entry.file_name());

        let mut templates = Vec::new();
        let mut outputs: HashMap<String, String> = HashMap::new();
        for entry in entries {
            let path = entry.path();
            let file_name = match entry.file_name().to_str() {
                Some(name) if name.starts_with('.') => continue,
                Some(name) => name.to_string(),
                None => continue,
            };

            if path.is_dir() {
                let is_dest = self.dest_root.as_deref() == Some(absolute_path(&path).as_path());
                if self.args.recursive && !is_dest {
                    self.process(&path, dest_dir.as_ref().map(|dir| dir.join(&file_name)))?;
                }
                continue;
            }

            // page.html.tera -> page.html; refuse to let two templates race for one file
            let output = output_name(&file_name).to_string();
            if let Some(other) = outputs.insert(output.clone(), file_name.clone()) {
                return Err(TeraclioError::InvalidInput(format!(
                    "'{other}' and '{file_name}' in {} would both render to '{output}'",
                    template_dir.display()
                )));
            }
            templates.push((path, file_name, output));
        }

        // One engine per directory: every template is parsed once, not once
        // per sibling render.
        let mut engine = TemplateEngine::new();
        let loaded = engine.load_directory(template_dir, &self.sources);

        for (path, file_name, output) in templates {
            let output_path = dest_dir.as_ref().map(|dir| dir.join(&output));
            let rendered = if loaded.contains(&file_name) {
                engine.render_named(&file_name, self.json_data)?
            } else {
                // Did not parse: load it alone to surface the real error
                render_file(&path, self.json_data, &self.sources)?
            };
            self.differs |= handle_rendered_output(&rendered, output_path.as_deref(), self.args)?;
            info(self.args, &format!("[teraclio] Processed: {file_name}"));
        }

        Ok(())
    }
}

/**
 * Render once in file or directory mode. Returns whether --diff found changes.
 * @author: skitsanos
 */
fn execute(args: &Cli) -> Result<bool> {
    let template_path = Path::new(require_template_path(args)?);

    if template_path.is_dir() {
        let json_data = parse_data(args)?;
        return run_directory_mode(template_path, args, &json_data);
    }

    render_once(args)
}

/**
 * Get the required template path from CLI args
 * @author: skitsanos
 */
fn require_template_path(args: &Cli) -> Result<&OsString> {
    args.template_path
        .as_ref()
        .ok_or_else(|| TeraclioError::InvalidInput("--template is required".to_string()))
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
fn run() -> Result<ExitCode> {
    let args = Cli::parse();

    // Generate shell completions and exit
    if let Some(shell) = args.completions {
        generate_completions(shell);
        return Ok(ExitCode::SUCCESS);
    }

    // List all available filters and exit
    if args.list_filters {
        let width = FILTERS
            .iter()
            .map(|spec| spec.name.len())
            .chain(FUNCTIONS.iter().map(|(name, _)| name.len() + 2))
            .max()
            .unwrap_or(0);
        println!("Available filters ({}):\n", FILTERS.len());
        for spec in FILTERS {
            println!("  {:<width$}  {}", spec.name, spec.description);
        }
        println!("\nAvailable functions ({}):\n", FUNCTIONS.len());
        for (name, description) in FUNCTIONS {
            println!("  {:<width$}  {description}", format!("{name}()"));
        }
        return Ok(ExitCode::SUCCESS);
    }

    let template_path = Path::new(require_template_path(&args)?);

    if !args.watch {
        let differs = execute(&args)?;
        if args.check {
            info(&args, "Template render check passed.");
        }
        // Like diff(1): exit 1 when --diff found changes, so CI can gate on drift
        return Ok(if differs {
            ExitCode::from(1)
        } else {
            ExitCode::SUCCESS
        });
    }

    // Reject stdin before rendering, so the data is not consumed for nothing
    if args.json_source.iter().any(|source| source == "-") {
        return Err(TeraclioError::InvalidInput(
            "Watch mode cannot be used with stdin ('-') as source.".to_string(),
        ));
    }

    // In watch mode a broken first render is reported, not fatal: the user is
    // about to fix the template or data, and the watcher should pick that up.
    match execute(&args) {
        Ok(_) if args.check => info(&args, "Template render check passed."),
        Ok(_) => {}
        Err(e) => eprintln!("[teraclio] Render error: {e}"),
    }

    watch(&args, template_path).map(|()| ExitCode::SUCCESS)
}

/**
 * What watch mode listens to, and which events trigger a re-render.
 * @author: skitsanos
 */
struct WatchPlan {
    /// Directories to watch. Watching directories rather than files survives
    /// editors that save by writing a temp file and renaming it over the
    /// original, which silently drops a watch placed on the file itself.
    dirs: Vec<(PathBuf, RecursiveMode)>,
    /// Exact files that trigger a re-render: the template and every source.
    files: HashSet<PathBuf>,
    /// Any non-hidden file here triggers a re-render: include/extends
    /// siblings of a template file, or every template in directory mode.
    template_dir: (PathBuf, RecursiveMode),
    /// The rendered output (file or directory). Its changes must never
    /// trigger a re-render, or every write would schedule the next one.
    output: Option<PathBuf>,
}

impl WatchPlan {
    fn new(args: &Cli, template_path: &Path) -> Self {
        let template_abs = absolute_path(template_path);
        let mut files = HashSet::new();

        let template_dir = if template_path.is_dir() {
            let mode = if args.recursive {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            };
            (template_abs, mode)
        } else {
            let dir = parent_dir(&template_abs);
            files.insert(template_abs);
            (dir, RecursiveMode::NonRecursive)
        };

        let mut dirs = vec![template_dir.clone()];
        for source in &args.json_source {
            let source_abs = absolute_path(Path::new(source));
            let dir = parent_dir(&source_abs);
            let covered = dirs.iter().any(|(watched, mode)| {
                *watched == dir || (*mode == RecursiveMode::Recursive && dir.starts_with(watched))
            });
            if !covered {
                dirs.push((dir, RecursiveMode::NonRecursive));
            }
            files.insert(source_abs);
        }

        let output = args
            .output_file
            .as_ref()
            .map(|output| absolute_path(Path::new(output)));

        Self {
            dirs,
            files,
            template_dir,
            output,
        }
    }

    fn is_relevant(&self, path: &Path) -> bool {
        if self
            .output
            .as_ref()
            .is_some_and(|output| path.starts_with(output))
        {
            return false;
        }
        if self.files.contains(path) {
            return true;
        }

        // Skip dotfiles and editor scratch files (.swp, foo~, .#lock)
        let scratch = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_none_or(|name| name.starts_with('.') || name.ends_with('~'));
        if scratch {
            return false;
        }

        let (dir, mode) = &self.template_dir;
        match mode {
            RecursiveMode::Recursive => path.starts_with(dir),
            RecursiveMode::NonRecursive => path.parent() == Some(dir.as_path()),
        }
    }
}

/**
 * Directory containing an absolute path (the path itself if it has no parent).
 * @author: skitsanos
 */
fn parent_dir(path: &Path) -> PathBuf {
    path.parent().unwrap_or(path).to_path_buf()
}

/**
 * Watch the template and sources, re-rendering after each burst of changes.
 * @author: skitsanos
 */
fn watch(args: &Cli, template_path: &Path) -> Result<()> {
    /// Quiet period that ends a burst; one editor save emits several events.
    const DEBOUNCE: Duration = Duration::from_millis(150);
    /// Upper bound on how long a continuous stream of events can delay a render.
    const MAX_DEBOUNCE: Duration = Duration::from_secs(1);

    let plan = WatchPlan::new(args, template_path);
    let (tx, rx) = mpsc::channel();
    let mut watcher = recommended_watcher(tx)?;
    for (dir, mode) in &plan.dirs {
        watcher.watch(dir, *mode)?;
    }

    info(args, "[teraclio] Watching for changes...");

    let changed_path = |event: notify::Result<Event>| -> Option<PathBuf> {
        match event {
            Ok(event)
                if matches!(
                    event.kind,
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                ) =>
            {
                event.paths.into_iter().find(|path| plan.is_relevant(path))
            }
            Ok(_) => None,
            Err(e) => {
                eprintln!("[teraclio] Watch error: {e}");
                None
            }
        }
    };

    loop {
        let first = rx
            .recv()
            .map_err(|e| TeraclioError::WatchError(format!("Watch channel closed: {e}")))?;
        let mut trigger = changed_path(first);

        // Drain the rest of the burst so one save renders once, but never
        // postpone a render indefinitely if the directory is never quiet.
        let deadline = Instant::now() + MAX_DEBOUNCE;
        while Instant::now() < deadline {
            match rx.recv_timeout(DEBOUNCE) {
                Ok(event) if trigger.is_none() => trigger = changed_path(event),
                Ok(_) => {}
                Err(_) => break,
            }
        }

        if let Some(path) = trigger {
            info(
                args,
                &format!(
                    "[teraclio] Detected change in {}, re-rendering...",
                    path.display()
                ),
            );
            if let Err(e) = execute(args) {
                eprintln!("[teraclio] Re-render error: {e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{absolute_path, set_data_path, Cli, WatchPlan};
    use clap::Parser;
    use serde_json::json;
    use std::path::Path;

    #[test]
    fn set_data_path_nests_and_preserves_siblings() {
        let mut data = json!({"app": {"port": 80}});
        set_data_path(&mut data, "app.db.host", json!("pg")).unwrap();
        set_data_path(&mut data, "top", json!(1)).unwrap();
        assert_eq!(
            data,
            json!({"app": {"port": 80, "db": {"host": "pg"}}, "top": 1})
        );
    }

    #[test]
    fn set_data_path_rejects_bad_keys_and_non_objects() {
        let mut data = json!({"app": "string"});
        assert!(set_data_path(&mut data, "app.port", json!(1)).is_err());
        assert!(set_data_path(&mut data, ".x", json!(1)).is_err());
        assert!(set_data_path(&mut data, "x.", json!(1)).is_err());
        assert!(set_data_path(&mut json!([1]), "x", json!(1)).is_err());
    }

    fn plan(args: &[&str]) -> WatchPlan {
        let cli = Cli::parse_from(std::iter::once("teraclio").chain(args.iter().copied()));
        let template = cli.template_path.clone().expect("template");
        WatchPlan::new(&cli, Path::new(&template))
    }

    #[test]
    fn output_next_to_template_never_triggers() {
        let plan = plan(&[
            "-s",
            "work/d.json",
            "-t",
            "work/t.txt",
            "-d",
            "work/out.txt",
        ]);
        assert!(!plan.is_relevant(&absolute_path(Path::new("work/out.txt"))));
        assert!(plan.is_relevant(&absolute_path(Path::new("work/t.txt"))));
        assert!(plan.is_relevant(&absolute_path(Path::new("work/d.json"))));
        // Siblings can be included, so they trigger too
        assert!(plan.is_relevant(&absolute_path(Path::new("work/header.txt"))));
    }

    #[test]
    fn editor_scratch_files_and_unrelated_source_siblings_are_ignored() {
        let plan = plan(&["-s", "data/d.json", "-t", "tpl/t.txt"]);
        assert!(!plan.is_relevant(&absolute_path(Path::new("tpl/.t.txt.swp"))));
        assert!(!plan.is_relevant(&absolute_path(Path::new("tpl/t.txt~"))));
        assert!(!plan.is_relevant(&absolute_path(Path::new("data/other.json"))));
        assert!(plan.is_relevant(&absolute_path(Path::new("data/d.json"))));
    }

    #[test]
    fn source_directories_are_watched_not_source_files() {
        // Watching the directory survives atomic rename-over saves
        let plan = plan(&["-s", "data/d.json", "-t", "tpl/t.txt"]);
        let dirs: Vec<_> = plan.dirs.iter().map(|(dir, _)| dir.clone()).collect();
        assert!(dirs.contains(&absolute_path(Path::new("data"))));
        assert!(dirs.contains(&absolute_path(Path::new("tpl"))));
    }

    #[test]
    fn directory_mode_ignores_destination_inside_template_tree() {
        let plan = plan(&["-s", "d.json", "-t", ".", "-d", "out", "-r"]);
        assert!(!plan.is_relevant(&absolute_path(Path::new("out/a.txt"))));
        assert!(plan.is_relevant(&absolute_path(Path::new("sub/a.txt"))));
    }
}
