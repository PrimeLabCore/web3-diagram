use scanner_syn;
use scanner_syn::contract_descriptor::{DefaultContractDescriptor, ContractDescriptor};

use clap::{Command, Arg};
use subprocess::{Popen, PopenConfig};

use std::env;
use std::path::Path;

fn main() -> Result<(), subprocess::PopenError> {
    let matches = Command::new("cargo-diagram")
        /*.arg(Arg::new("path")
            .short('p')
            .long("path")
            .required(false)
            .takes_value(true)
            .help("Path to the Rust project. Must contain Cargo.toml file. Optional"))*/
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
            .help("Output file. It should be either md, svg, png or pdf. Optional.
                Default: \"./res/name_of_the_input_file.svg\""))
        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .required(false)
            .takes_value(true)
            .requires("input")
            .conflicts_with("output")
            .help("Format of the output file. Can be used if the output is not provided. 
            Output name will be name_of_the_input_file and it will be placed at ./res folder. Options"))
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
        .arg(Arg::new("backgroundColor")
            .short('b')
            .long("backgroundColor")
            .required(false)
            .takes_value(true)
            .requires("input")
            .help("Background color. Example: transparent, red, '#F0F0F0'. Optional. Default: white"))
        .arg(Arg::new("quiet")
            .short('q')
            .long("quiet")
            .required(false)
            .takes_value(false)
            .help("Suppress log output"))
        .get_matches();

    /*if let Some(path) = matches.value_of("path") {
        let current_path = env::current_dir()?;
        env::set_current_dir(&path)?;
        let descriptor = DefaultContractDescriptor::new();
        let info = descriptor.get_contract_info_for_crate();
        /*println!("{}", info.functions.len());
        for index in 0..info.functions.len() {
            println!("{}", info.functions[index].name)
        }*/
        env::set_current_dir(&current_path)?;
    }*/

    let is_quiet = matches.is_present("quiet");

    let input_file = matches.value_of("input").unwrap();
    let mut command = vec!["mmdc", "-i", input_file];
    let mut full_path: String;
    if let Some(output_file) = matches.value_of("output") {
        command.push("-o");
        command.push(output_file);
    } else {
        command.push("-o");
        let mut path = env::current_dir()?;
        path.push("res/");
        let input_vec: Vec<&str> = input_file.rsplit_terminator(&['.', '/'][..]).collect();
        let output_name = match input_vec[0] {
            "md" => input_vec[1].to_owned(),
            _ => input_vec[0].to_owned(),
        };
        let output_extension = match matches.value_of("format") {
            Some(extension) => ".".to_owned() + extension,
            _ => ".svg".to_string(),
        };
        let path_output = output_name + &output_extension;
        full_path = (path.to_str().unwrap().to_owned() + &path_output).to_string();
        command.push(&full_path.as_str());
    };
    if let Some(height) = matches.value_of("height") {
        if !is_quiet {
            println!("Set the height: {}", height);
        };
        command.push("-H");
        command.push(height);
    };
    if let Some(width) = matches.value_of("width") {
        if !is_quiet {
            println!("Set the width: {}", width);
        };        
        command.push("-w");
        command.push(width);
    };
    if let Some(scale) = matches.value_of("scale") {
        if !is_quiet {
            println!("Set the scale: {}", scale);
        };  
        command.push("-s");
        command.push(scale);
    };
    if let Some(background_color) = matches.value_of("backgroundColor") {
        if !is_quiet {
            println!("Set the background color: {}", background_color);
        };
        command.push("-b");
        command.push(background_color);
    };
    if is_quiet {
        command.push("-q");
    }
    let mut mmdc = Popen::create(&command, PopenConfig::default())?;

    mmdc.wait();

    Ok(())
}
