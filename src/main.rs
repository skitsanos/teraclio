use crate::cli::Cli;
use crate::engine::TemplateEngine;
use crate::error::Result;
use crate::utils::parse_json_source;
use clap::Parser;

mod cli;
mod engine;
mod error;
mod filters;
mod utils;

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
 * Main application logic with proper error handling
 * @author: skitsanos
 */
fn run() -> Result<()> {
    let args = Cli::parse();

    let json_data = parse_json_source(&args.json_source)?;

    let mut engine = TemplateEngine::new();
    engine.load_template(&args.template_path)?;

    let rendered = engine.render(&args.template_path, &json_data)?;

    let output_path = args
        .output_file
        .as_ref()
        .map(|p| p.as_ref() as &std::path::Path);
    TemplateEngine::write_output(&rendered, output_path)?;

    Ok(())
}
