use std::fs::File;
use std::io::{self, Write};
use std::path::{PathBuf};
use std::ffi::OsString;
use structopt::StructOpt;
use tera::{Context, Tera};
use crate::base64decode::filter_base64_decode;
use crate::base64encode::filter_base64_encode;
use crate::bytes2str::filter_bytes_to_str;
use crate::str2bytes::filter_str_to_bytes;
use crate::utils::parse_json_source;

mod utils;
mod base64encode;
mod base64decode;
mod bytes2str;
mod str2bytes;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long = "template", short = "t", parse(from_os_str))]
    template_path: OsString,

    #[structopt(long = "dest", short = "d", parse(from_os_str))]
    output_file: OsString,

    #[structopt(long = "source", short = "s", parse(from_os_str))]
    json_source: PathBuf,
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

    tera.register_filter("base64_encode", filter_base64_encode);
    tera.register_filter("base64_decode", filter_base64_decode);
    tera.register_filter("bytes_to_str", filter_bytes_to_str);
    tera.register_filter("str_to_bytes", filter_str_to_bytes);

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