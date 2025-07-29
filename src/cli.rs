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

    #[arg(long = "source", short = 's', help = "Path to the JSON source file")]
    pub json_source: PathBuf,
}
