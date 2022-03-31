use minidom;
use scanner_syn;

use minidom::Element;
use scanner_syn::contract_descriptor::{ContractDescriptor, DefaultContractDescriptor};

use clap::{Arg, Command};
use subprocess::{Communicator, Popen, PopenConfig, Redirection};

use std::env;
use std::fs::{self, File};
use std::io::Read;
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
            Output name will be name_of_the_input_file and it will be placed at ./res folder. Optional"))
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
    let full_output_path: String;
    if let Some(output_file) = matches.value_of("output") {
        command.push("-o");
        full_output_path = output_file.clone().into();
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
        let path_output = output_name + &".svg";
        full_output_path = (path.to_str().unwrap().to_owned() + &path_output).to_string();
        command.push(&full_output_path.as_str());
    };
    /*if let Some(height) = matches.value_of("height") {
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
    };*/
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

    let mut mmdc = Popen::create(
        &command,
        PopenConfig {
            stdout: Redirection::Pipe,
            ..PopenConfig::default()
        },
    )?;
    mmdc.wait();

    let (output, _) = mmdc.communicate(None).unwrap();
    let split_output_lines: Vec<&str> = output
        .as_ref()
        .unwrap()
        .split_terminator(&['\n'][..])
        .collect();
    let mut output_files: Vec<String> = vec![];
    for line in split_output_lines {
        // ✅ U+2705
        if let Some(start) = line.find(" \u{2705}") {
            if start == 0 {
                if !is_quiet {
                    println!("Created file {}", &line.replacen(" \u{2705} ", "", 1));
                };
                output_files.push((&line.replacen(" \u{2705} ", "", 1)).to_string());
            };
        }
    }

    let height = matches.value_of("height").unwrap_or("600");
    let width = matches.value_of("width").unwrap_or("800");
    for output_file in output_files {
        let contents = fs::read_to_string(output_file.as_str())
            .expect("Something went wrong reading the file");
        let mut root: Element = contents.parse().unwrap();
        let mut style: String = String::from(root.attr("style").unwrap_or(""));
        style += format!(" max-width: {}px;", width).as_str();
        root.set_attr("height", height);
        root.set_attr("width", width);
        root.set_attr("style", style);
        let mut out_file = File::create(output_file.as_str())?;
        root.write_to(&mut out_file);
    }

    let format = match matches.value_of("format") {
        Some(format) => {
            assert!(vec!["svg", "png", "pdf", "md"].contains(&format), "Incorrect output format");
            format
        },
        None => {
            if let Some(output) = matches.value_of("output") {
                let split: Vec<&str> = output.rsplit_terminator(&['.'][..]).collect();
                assert!(
                    vec!["svg", "png", "pdf", "md"].contains(&split[0]),
                    "Incorrect output format"
                );
                split[0]
            } else {
                "svg"
            }
        },
    };
    if !is_quiet {
        println!("Set the format: {}", format);
    };

    Ok(())
}
