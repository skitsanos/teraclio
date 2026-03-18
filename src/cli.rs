use crate::utils::InputFormat;
use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
#[command(name = "teraclio")]
#[command(about = "A CLI tool for template rendering with Tera")]
#[command(version)]
pub struct Cli {
    #[arg(long = "template", short = 't', help = "Path to the template file or directory")]
    pub template_path: OsString,

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
        required = true,
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
}
