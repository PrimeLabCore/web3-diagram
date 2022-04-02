//! The command line tool which creates diagrams from markdown files.
//! Based on [mermaid-cli](https://github.com/mermaid-js/mermaid-cli).
//!
//! In order to create a diagram one must provide the full path to the input file.
//! Optional parameters are output, format, scale, height, width, background color, quiet.
//!
//! For more detailed info run with `--help` or `-h` flag.
use minidom;
//use scanner_syn;
use minidom::Element;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use clap::{Arg, Command};
use subprocess::{Popen, PopenConfig, Redirection};
// use scanner_syn::contract_descriptor::{ContractDescriptor, DefaultContractDescriptor};

use mermaid_markdown_api::scanner_pipeline::ScannerPipeline;
use mermaid_markdown_api::syntax::FlowDirection;
use scanner_syn::contract_descriptor::{ContractDescriptor, DefaultContractDescriptor};
use std::env;
use std::fs::{self, File};

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
            .help("Output file. It should be either md, svg, png or pdf. Optional. Default: \"./res/name_of_the_input_file.svg\""))
        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .required(false)
            .takes_value(true)
            .requires("input")
            .conflicts_with("output")
            .help("Format of the output file. Can be used if the output is not provided. Output name will be name_of_the_input_file and it will be placed at ./res folder. Optional"))
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
            .help("Puppeteer scale factor. Optional. Default: 1"))
        .arg(Arg::new("backgroundColor")
            .short('b')
            .long("backgroundColor")
            .required(false)
            .takes_value(true)
            .requires("input")
            .help("Background color. Example: transparent, red, '#F0F0F0'. Optional. Default: white"))
        .arg(Arg::new("openb")
            .short('O')
            .long("openb")
            .required(false)
            .takes_value(false)
            .requires("input")
            .help("Should open output file in browser"))
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

    // Determine whether the program should output some info
    let is_quiet = matches.is_present("quiet");

    // Determine the format of the output
    let format = match matches.value_of("format") {
        Some(format) => {
            assert!(
                vec!["svg", "png", "pdf", "md"].contains(&format),
                "Incorrect output format"
            );
            format
        }
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
        }
    };
    if !is_quiet {
        println!("Set the format: {}", format);
    };

    // Determine the input file
    let input_file_name = matches.value_of("input").unwrap();

    let input_file_path: PathBuf = create_markdown_file(input_file_name).unwrap();
    let mut command = vec!["npx", "mmdc", "-i", input_file_path.to_str().unwrap()];

    let mut output_path = input_file_path.clone();
    output_path.set_extension("svg");
    let output_name = output_path.file_stem().unwrap();

    // Set the output file
    if let Some(output_file) = matches.value_of("output") {
        // command.push("-o");
        // command.push(output_file);
    } else {
        command.push("-o");
        command.push(output_path.to_str().unwrap());
    }

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

    // Check if the scale is provided
    if let Some(scale) = matches.value_of("scale") {
        if !is_quiet {
            println!("Set the scale: {}", scale);
        };
        command.push("-s");
        command.push(scale);
    };
    // Check if the background color is provided
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

    // Call mermaid-cli with the constracted command, create svg file
    let mut mmdc = Popen::create(
        &command,
        PopenConfig {
            stdout: Redirection::Pipe,
            ..PopenConfig::default()
        },
    )?;
    let _ = mmdc.wait();

    // // List all of the created files
    // let (output, err) = mmdc.communicate(None).unwrap();
    // let split_output_lines: Vec<&str> = output.as_ref().unwrap().split('\n').collect();
    // let mut output_files: Vec<String> = vec![];
    // for line in split_output_lines {
    //     // ✅ U+2705
    //     if let Some(start) = line.find(" \u{2705}") {
    //         if start == 0 {
    //             if !is_quiet {
    //                 println!("Created file {}", &line.replacen(" \u{2705} ", "", 1));
    //             };
    //             output_files.push((&line.replacen(" \u{2705} ", "", 1)).to_string());
    //         };
    //     }
    // }

    // // Change the height and the width of the created file to the amount, which were provided
    // let height = matches.value_of("height").unwrap_or("600");
    // let width = matches.value_of("width").unwrap_or("800");
    // for output_file in output_files.iter() {
    //     let contents = fs::read_to_string(output_file.as_str())
    //         .expect("Something went wrong reading the file");
    //     let mut root: Element = contents.parse().unwrap();
    //     let mut style: String = String::from(root.attr("style").unwrap_or(""));
    //     style += format!(" max-width: {}px;", width).as_str();
    //     root.set_attr("height", height);
    //     root.set_attr("width", width);
    //     root.set_attr("style", style);
    //     let mut out_file = File::create(output_file.as_str())?;
    //     root.write_to(&mut out_file).unwrap();
    // }

    // TODO: Create the output files with the given extension from the svg file
    let mut opt = usvg::Options {
        resources_dir: std::fs::canonicalize(output_path.clone())
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf())),
        ..Default::default()
    };
    opt.fontdb.load_system_fonts();
    println!("{}", input_file_name);
    let svg_data = std::fs::read(output_path.clone()).unwrap();

    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap();
    match format {
        "svg" => {}
        "png" => {
            let pixmap_size = rtree.svg_node().size.to_screen_size();
            let mut pixmap =
                tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
            resvg::render(
                &rtree,
                usvg::FitTo::Original,
                tiny_skia::Transform::default(),
                pixmap.as_mut(),
            )
            .unwrap();
            let mut png_path = env::current_dir()?;
            png_path.push("res/");
            png_path.push(output_name);
            png_path.set_extension("png");
            pixmap.save_png(png_path).unwrap();
        }
        "pdf" => {
            let pdf = svg2pdf::convert_tree(&rtree, svg2pdf::Options::default());
            let mut pdf_path = env::current_dir()?;
            pdf_path.push("res/");
            pdf_path.push(output_name);
            pdf_path.set_extension("pdf");
            std::fs::write(pdf_path, pdf).unwrap();
        }
        "md" => {}
        _ => unreachable!(),
    };

    let should_open_in_browser = matches.is_present("openb");
    if should_open_in_browser {
        open_output_file_in_browser(output_path);
    }
    Ok(())
}

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
fn create_markdown_file(file_name: &str) -> Result<PathBuf, std::io::Error> {
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
