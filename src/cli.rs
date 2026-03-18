use crate::utils::InputFormat;
use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
#[command(name = "teraclio")]
#[command(about = "A CLI tool for template rendering with Tera")]
#[command(version)]
pub struct Cli {
    #[arg(long = "template", short = 't', help = "Path to the template file")]
    pub template_path: OsString,

    #[arg(
        long = "dest",
        short = 'd',
        help = "Output file path (stdout if not specified)"
    )]
    pub output_file: Option<OsString>,

    #[arg(
        long = "source",
        short = 's',
        allow_hyphen_values = true,
        help = "Path to the data source file (JSON, YAML, or TOML), or '-' for stdin"
    )]
    pub json_source: String,

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
}
