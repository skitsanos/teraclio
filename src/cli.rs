use crate::utils::InputFormat;
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use std::ffi::OsString;

#[derive(Parser)]
#[command(name = "teraclio")]
#[command(about = "A CLI tool for template rendering with Tera")]
#[command(version)]
pub struct Cli {
    #[arg(long = "completions", value_enum, help = "Generate shell completions and exit")]
    pub completions: Option<Shell>,

    #[arg(long = "list-filters", help = "List all available template filters and exit")]
    pub list_filters: bool,
    #[arg(long = "template", short = 't', help = "Path to the template file or directory", required_unless_present_any = ["completions", "list_filters"])]
    pub template_path: Option<OsString>,

    #[arg(
        long = "dest",
        short = 'd',
        help = "Output file path (stdout if not specified), or directory when using directory mode"
    )]
    pub output_file: Option<OsString>,

    #[arg(
        long = "source",
        short = 's',
        allow_hyphen_values = true,
        help = "Path to data source file(s) (JSON, YAML, or TOML), or '-' for stdin. Can be specified multiple times.",
        num_args = 1,
        required_unless_present_any = ["completions", "list_filters"],
    )]
    pub json_source: Vec<String>,

    #[arg(
        long = "format",
        short = 'f',
        help = "Input format (json, yaml, toml) - auto-detected for files, required for stdin",
        value_enum
    )]
    pub input_format: Option<InputFormat>,

    #[arg(
        long = "env-vars",
        help = "Include environment variables in template data as 'env' object"
    )]
    pub include_env_vars: bool,

    #[arg(long = "check", help = "Validate the template without rendering")]
    pub check: bool,

    #[arg(long = "strict", help = "Fail on undefined template variables")]
    pub strict: bool,

    #[arg(long = "watch", short = 'w', help = "Watch source and template files for changes and re-render")]
    pub watch: bool,

    #[arg(
        long = "set",
        value_name = "KEY=VALUE",
        help = "Set a template variable (can be used multiple times)",
        num_args = 1,
    )]
    pub set_vars: Vec<String>,

    #[arg(long = "quiet", short = 'q', help = "Suppress informational messages on stderr")]
    pub quiet: bool,
}

/**
 * Generate shell completions and write to stdout
 * @author: skitsanos
 */
pub fn generate_completions(shell: Shell) {
    let mut cmd = Cli::command();
    clap_complete::generate(shell, &mut cmd, "teraclio", &mut std::io::stdout());
}
