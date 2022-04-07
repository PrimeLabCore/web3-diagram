//! The command line tool which creates diagrams from markdown files.
//! Based on [mermaid-cli](https://github.com/mermaid-js/mermaid-cli).
//!
//! In order to create a diagram one must provide the full path to the input file.
//! Optional parameters are output, format, scale, height, width, background color, quiet.
//!
//! For more detailed info run with `--help` or `-h` flag.
pub mod svg;
use minidom;
use svg::{load_from_data, load_from_path};
//use scanner_syn;
use minidom::Element;
use std::io::{Error, ErrorKind, self};
use std::path::{Path, PathBuf};
use usvg::{Node, NodeExt};

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
        println!("Adding logo");

        let _ = mmdc.wait();
        // self.add_logo(output_path.clone());
        Ok(output_path)
    }
    fn add_logo(&self, output_path: PathBuf) {
        let tree = load_from_path(output_path.as_path()).unwrap();
        let logo_data = include_bytes!("logo.svg");

        let logo_tree = load_from_data(logo_data).unwrap();

        let mut tree_root = tree.root();
        let logo_root = logo_tree.root();

        tree_root.prepend(logo_root);

        // println!("{:?}", tree_root.into());
        // File::create(&output_path)
        //     .unwrap()

        //     .write_all(tree.root().())
        //     .unwrap();
    }
}

fn main() -> Result<(), subprocess::PopenError> {
    let args = Cli::parse();
    let output_path = args.pass_to_mmdc()?;

    // println!(
    //     "{}",
    //     args.input_file.into_os_string().into_string().unwrap()
    // );

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
    println!("opening {:?}", output_path.to_str().unwrap());
    let command = vec!["open", "-a", "Google Chrome", output_path.to_str().unwrap()];

    let mut executor = Popen::create(
        &command,
        PopenConfig {
            stdout: Redirection::Pipe,
            ..PopenConfig::default()
        },
    )
    .unwrap();

    let _ = executor
        .wait()
        .expect("Could not open file in google chrome, try open it manualiy");
}

/// Function creates markdown file with specified file name
///
/// # Arguments
///
/// * `file_name` - Markdown file name
fn create_markdown_file(file_name: PathBuf) -> Result<PathBuf, std::io::Error> {
    let mut current_dir = env::current_dir().expect("Can not resolve current directory");
   
    if current_dir.ends_with("res") || current_dir.ends_with("src"){
        current_dir.pop();
    }
    if !does_folder_exist_in_directory("src",current_dir.clone()).unwrap(){
        panic!("You are not in crate dir");
    }

    let desc = DefaultContractDescriptor::new();
    let contract_info = desc.get_contract_info_for_crate(current_dir.clone().to_str());
    let markdown = ScannerPipeline::from(contract_info, FlowDirection::TD);
    //println!("{:?}", markdown.content);

    if !current_dir.ends_with("res") {
        current_dir.push("res/");
    }
    std::fs::create_dir_all(current_dir.clone())?;
    current_dir.push(file_name);
    fs::write(current_dir.clone(), markdown.content).expect("Unable to write file");
    Ok(current_dir.clone())
}
fn does_folder_exist_in_directory(folder:&str,directory_path:PathBuf) -> io::Result<bool> {
    let mut dir=directory_path.clone();
    dir.push(folder);
    let metadata = fs::metadata(dir).expect("You are not in crate directory!");
    Ok(metadata.is_dir())
}