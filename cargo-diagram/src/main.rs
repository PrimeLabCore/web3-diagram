//! The command line tool which creates diagrams from markdown files.
//! Based on [mermaid-cli](https://github.com/mermaid-js/mermaid-cli).
//!
//! In order to create a diagram one must provide the full path to the input file.
//! Optional parameters are output, format, scale, height, width, background color, quiet.
//!
//! For more detailed info run with `--help` or `-h` flag.
pub mod svg;
use svg::{load_from_data,load_from_path};
use minidom;
//use scanner_syn;
use minidom::Element;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use clap::Parser;
use subprocess::{Popen, PopenConfig, Redirection};
// use scanner_syn::contract_descriptor::{ContractDescriptor, DefaultContractDescriptor};

use mermaid_markdown_api::scanner_pipeline::ScannerPipeline;
use mermaid_markdown_api::syntax::FlowDirection;
use scanner_syn::contract_descriptor::{ContractDescriptor, DefaultContractDescriptor};
use std::env;
use std::fs::{self, File};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to the markdown file with the input data
    #[clap(short, long = "input", parse(from_os_str), value_name = "FILE")]
    input_file: PathBuf,
    /// Output file. It should be either md, svg, png or pdf. Optional. Default: \"./res/name_of_the_input_file.svg\
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    output_file: Option<PathBuf>,
    /// Scale factor. Optional. Default: 1
    #[clap(short, long, value_name = "SCALE")]
    scale: Option<String>,
    /// Height of the page. Optional. Default: 600
    #[clap(short, long, value_name = "HEIGHT")]
    height: Option<String>,
    /// Width of the page. Optional. Default: 800
    #[clap(short, long, value_name = "WIDTH")]
    width: Option<String>,
    /// Background color. Example: transparent, red, '#F0F0F0'. Optional. Default: white
    #[clap(short, long, value_name = "COLOR")]
    background_color: Option<String>,
    /// Should open output file in browser
    #[clap(short = 'O', long)]
    openb: bool,
    /// Suppress log output
    #[clap(short, long)]
    quiet: bool,
}

impl Cli {
    /// Method to pass arguments to the mermaid-cli
    /// 
    /// Returns created file path
    fn pass_to_mmdc(&self) -> Result<PathBuf, subprocess::PopenError> {
        let input_file_path: PathBuf = create_markdown_file(self.input_file.clone()).unwrap();

        let mut command = vec!["mmdc", "-i", input_file_path.to_str().unwrap()];

        let output_path = if let Some(output_file) = &self.output_file {
            output_file.clone()
        } else {
            let mut output_path = input_file_path.clone();
            output_path.set_extension("svg");
            output_path
        };
        // Set the output file
        command.push("-o");
        command.push(output_path.to_str().unwrap());

        // Check if the scale is provided
        if let Some(scale) = &self.scale {
            if !self.quiet {
                println!("Set the scale: {}", scale);
            };
            command.push("-s");
            command.push(scale);
        };
        // Check if the height is provided
        if let Some(height) = &self.height {
            if !self.quiet {
                println!("Set the height: {}", height);
            };
            command.push("-h");
            command.push(height);
        };
        // Check if the width is provided
        if let Some(width) = &self.width {
            if !self.quiet {
                println!("Set the width: {}", width);
            };
            command.push("-w");
            command.push(width);
        };
        // Check if the background color is provided
        if let Some(background_color) = &self.background_color {
            if !self.quiet {
                println!("Set the background color: {}", background_color);
            };
            command.push("-b");
            command.push(background_color);
        };
        if self.quiet {
            command.push("-q");
        }

        // Call mermaid-cli with the constracted command, create svg file
        let mut mmdc = Popen::create(
            &command,
            PopenConfig {
                stdout: Redirection::Pipe,
                ..PopenConfig::default()
            },
        )?;
        let _ = mmdc.wait();
        Ok(output_path)
    }
}

fn main() -> Result<(), subprocess::PopenError> {
    let args = Cli::parse();
    let output_path = args.pass_to_mmdc()?;

    println!(
        "{}",
        args.input_file.into_os_string().into_string().unwrap()
    );

    if args.openb {
        open_output_file_in_browser(output_path);
    }
    Ok(())
}

/// Function opens output file in web browser
///
/// # Arguments
///
/// * `output_path` - Output file path to open in browser
fn open_output_file_in_browser(output_path: PathBuf) {
    let command = vec!["open", "-a", "Google Chrome", output_path.to_str().unwrap()];

    let mut executor = Popen::create(
        &command,
        PopenConfig {
            stdout: Redirection::Pipe,
            ..PopenConfig::default()
        },
    )
    .unwrap();
    let _ = executor.wait();
}

/// Function creates markdown file with specified file name
///
/// # Arguments
///
/// * `file_name` - Markdown file name
fn create_markdown_file(file_name: PathBuf) -> Result<PathBuf, std::io::Error> {
    let desc = DefaultContractDescriptor::new();
    let contract_info = desc.get_contract_info_for_crate();
    let markdown = ScannerPipeline::from(contract_info, FlowDirection::TD);
    //println!("{:?}", markdown.content);

    let mut path = env::current_dir().expect("Can not resolve current directory");
    path.push("res/");
    std::fs::create_dir_all(path.clone())?;
    path.push(file_name);
    fs::write(path.clone(), markdown.content).expect("Unable to write file");
    Ok(path.clone())
}

