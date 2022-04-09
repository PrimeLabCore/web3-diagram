//! The command line tool which creates diagrams from markdown files.
//! Based on [mermaid-cli](https://github.com/mermaid-js/mermaid-cli).
//!
//! In order to create a diagram one must provide the full path to the input file.
//! Optional parameters are output, format, scale, height, width, background color, quiet.
//!
//! For more detailed info run with `--help` or `-h` flag.
pub mod svg;
use svg::{load_from_data, load_from_path};
//use scanner_syn;
use clap::Parser;
use std::fmt::Display;
use std::io::{self, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use subprocess::{Popen, PopenConfig, Redirection};
// use scanner_syn::contract_descriptor::{ContractDescriptor, DefaultContractDescriptor};

use mermaid_markdown_api::scanner_pipeline::ScannerPipeline;
use mermaid_markdown_api::syntax::FlowDirection;
use scanner_syn::contract_descriptor::{ContractDescriptor, DefaultContractDescriptor};
use std::env;
use std::fs::{self, File};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub enum OutputFormat {
    Svg,
    Png,
    Pdf,
    Jpg,
}
impl FromStr for OutputFormat {
    type Err = ();

    fn from_str(input: &str) -> Result<OutputFormat, Self::Err> {
        match input {
            "Jpg" => Ok(OutputFormat::Jpg),
            "Pdf" => Ok(OutputFormat::Pdf),
            "Png" => Ok(OutputFormat::Png),
            "Svg" => Ok(OutputFormat::Svg),
            _ => Ok(OutputFormat::Svg),
        }
    }
}

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
    /// Output file format. Optional. Default: Svg
    // #[clap(short, long = "format", value_name = "FORMAT")]
    // format: Option<String>,
    /// Contract name. Optional. Default: Contract
    #[clap(short='n', long = "cname", value_name = "CNAME")]
    contract_name: Option<String>,
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
        let input_file_path: PathBuf = self.create_markdown_file(self.input_file.clone(),self.contract_name.clone()).unwrap();

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
    fn add_logo(&self, output_path: &mut PathBuf) {
        let mut logo_str = include_str!("logo.svg").to_owned();
        let shape_str = include_str!("shapes.svg");
        logo_str.push_str(shape_str);

        let mut contents =
            fs::read_to_string(output_path.clone()).expect("Something went wrong reading the file");

        let index_of_style = contents.find("<style").unwrap();
        contents.insert_str(index_of_style, logo_str.as_str());
        //println!("With text:\n{}", contents);
        File::create(output_path)
            .unwrap()
            .write_all(contents.as_bytes())
            .unwrap();
    }

    /// Function opens output file in web browser
    ///
    /// # Arguments
    ///
    /// * `output_path` - Output file path to open in browser
    fn open_output_file_in_browser(&self, output_path: PathBuf) {
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
    fn create_markdown_file(&self, file_name: PathBuf,contract_name:Option<String>) -> Result<PathBuf, std::io::Error> {
        let mut current_dir = env::current_dir().expect("Can not resolve current directory");

        if current_dir.ends_with("res") || current_dir.ends_with("src") {
            current_dir.pop();
        }
        if !self
            .does_folder_exist_in_directory("src", current_dir.clone())
            .unwrap()
        {
            panic!("You are not in crate dir");
        }

        let desc = DefaultContractDescriptor::new();
        let contract_info = desc.get_contract_info_for_crate(current_dir.clone().to_str(),contract_name);
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
    fn does_folder_exist_in_directory(
        &self,
        folder: &str,
        directory_path: PathBuf,
    ) -> io::Result<bool> {
        let mut dir = directory_path.clone();
        dir.push(folder);
        let metadata = fs::metadata(dir).expect("You are not in crate directory!");
        Ok(metadata.is_dir())
    }
}

fn main() -> Result<(), subprocess::PopenError> {
    let args = Cli::parse();
    let mut output_path = args.pass_to_mmdc()?;

    args.add_logo(&mut output_path);

    if args.openb {
        args.open_output_file_in_browser(output_path);
    }

    Ok(())
}
