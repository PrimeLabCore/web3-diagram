use scanner_syn;
use scanner_syn::contract_descriptor::{DefaultContractDescriptor, ContractDescriptor};

use clap::{Command, Arg};
use subprocess::{Popen, PopenConfig};

use std::{env, fs};
use std::path::Path;

fn main() -> Result<(), subprocess::PopenError> {
    let matches = Command::new("cargo-diagram")
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .required(false)
            .takes_value(true)
            .help("Path to the Rust project. Must contain Cargo.toml file. Optional"))
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .required(true)
            .takes_value(true)
            .help("Path to the markdown file with the input data"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .required(false)
            .takes_value(true)
            .requires("input")
            .help("Output file. It should be either md, svg, png or pdf. Optional. Default: \"./res/output.svg\""))
        .arg(Arg::new("height")
            .short('H')
            .long("height")
            .required(false)
            .takes_value(true)
            .requires("input")
            .help("Height of the page. Optional. Default: 600"))
        .arg(Arg::new("width")
            .short('w')
            .long("width")
            .required(false)
            .takes_value(true)
            .requires("input")
            .help("Width of the page. Optional. Default: 800"))
        .arg(Arg::new("scale")
            .short('s')
            .long("scale")
            .required(false)
            .takes_value(true)
            .requires("input")
            .help("Puppeteer scale factor, default 1. Optional"))
        .get_matches();

    if let Some(path) = matches.value_of("path") {
        let current_path = env::current_dir()?;
        env::set_current_dir(&path)?;
        let descriptor = DefaultContractDescriptor::new(); 
        let info = descriptor.get_contract_info_for_crate();
        println!("{}", info.functions.len());
        for index in 0..info.functions.len() {
            println!("{}", info.functions[index].name)
        }
        env::set_current_dir(&current_path)?;
    }

    let input_file = matches.value_of("input").unwrap();
    let mut command = vec!["mmdc", "-i", input_file];
    let mut path_output: String;
    if let Some(output_file) = matches.value_of("output") {
        command.push("-o");
        let path = Path::new(output_file);
        /*if !path.is_dir() {
            fs::create_dir(&path);
        };*/
        command.push(output_file);
    } else {
        command.push("-o");
        let mut path = env::current_dir()?;
        path.push("res/");
        if !path.is_dir() {
            fs::create_dir(&path);
        };
        path.push("output.svg");
        println!("{}", path.as_path().display());
        path_output = path.as_path().to_str().unwrap().to_string();
        command.push(path_output.as_str());
    };
    if let Some(height) = matches.value_of("height") {
        println!("Set the height: {}", height);
        command.push("-H");
        command.push(height);
    };
    if let Some(width) = matches.value_of("width") {
        println!("Set the width: {}", width);
        command.push("-w");
        command.push(width);
    };
    if let Some(scale) = matches.value_of("scale") {
        println!("Set the scale: {}", scale);
        command.push("-s");
        command.push(scale);
    };
    let mut mmdc = Popen::create(&command, PopenConfig::default())?;
        // stdout: Redirection::File(std::io::stdout()),
        // stderr: Redirection::File(std::io::stderr()),
    mmdc.wait();

    Ok(())
}
