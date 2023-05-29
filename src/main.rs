use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{PathBuf};
use std::ffi::OsString;
use serde_json::Value;
use structopt::StructOpt;
use tera::{Context, Tera};

#[derive(StructOpt)]
struct Cli {
    #[structopt(long = "template", short = "t", parse(from_os_str))]
    template_path: OsString,

    #[structopt(long = "dest", short = "d", parse(from_os_str))]
    output_file: OsString,

    #[structopt(long = "source", short = "s", parse(from_os_str))]
    json_source: PathBuf,
}

fn parse_json_source(source_path: &PathBuf) -> Result<Value, Box<dyn std::error::Error>> {
    // Read the contents of the JSON file
    let mut file = File::open(source_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON contents into a serde_json::Value
    let json_value: Value = serde_json::from_str(&contents)?;

    Ok(json_value)
}

fn main() {
    // Parse command-line arguments using StructOpt
    let args = Cli::from_args();

    // Determine the JSON data from the source file
    let json_data = match parse_json_source(&args.json_source) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading JSON source file: {}", e);
            return;
        }
    };

    // Create a Tera instance and load the template from the specified file
    let mut tera = Tera::default();
    tera.add_template_file(args.template_path.to_string_lossy().as_ref(), None).unwrap();

    // Create a context with the JSON data
    let mut context = Context::new();
    context.insert("data", &json_data);

    // Render the template with the context
    let rendered = tera.render(args.template_path.to_string_lossy().as_ref(), &context).unwrap();

    // Write the rendered output to the specified file or stdout
    if let Some(output_file_path) = args.output_file.to_str() {
        // Write to the specified file
        let mut output_file = File::create(output_file_path).unwrap();
        output_file.write_all(rendered.as_bytes()).unwrap();
    } else {
        // Write to stdout
        io::stdout().write_all(rendered.as_bytes()).unwrap();
    }
}
