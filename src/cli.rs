use clap::Parser;
use std::ffi::OsString;
use std::path::PathBuf;

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
        help = "Path to the data source file (JSON, YAML, or TOML)"
    )]
    pub json_source: PathBuf,

    #[arg(
        long = "format",
        short = 'f',
        help = "Input format (json, yaml, toml) - auto-detected if not specified"
    )]
    pub input_format: Option<String>,

    #[arg(
        long = "env-vars",
        help = "Include environment variables in template data as 'env' object"
    )]
    pub include_env_vars: bool,
}
