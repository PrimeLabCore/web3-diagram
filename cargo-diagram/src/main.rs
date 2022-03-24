use scanner_syn;
use scanner_syn::contract_descriptor::{DefaultContractDescriptor, ContractDescriptor};

use clap::{Command, Arg};
use subprocess::{Popen, PopenConfig};

fn something() {
    5;
}

fn main() -> Result<(), subprocess::PopenError> {
    let descriptor = DefaultContractDescriptor::new();
    let info = descriptor.get_contract_info_for_crate();
    println!("{}", info.functions.len());
    println!("{:#?}", info.functions[0].name);
    println!("{:#?}", info.functions[1].name);
    let matches = Command::new("cargo-diagram")
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .required(true)
            .takes_value(true)
            .help("Path to the markdown file with the input data"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .required(false)
            .takes_value(true)
            .help("Output file. It should be either md, svg, png or pdf. Optional. Default: input + \".svg\""))
        .arg(Arg::new("height")
            .short('H')
            .long("height")
            .required(false)
            .takes_value(true)
            .help("Height of the page. Optional. Default: 600"))
        .arg(Arg::new("width")
            .short('w')
            .long("width")
            .required(false)
            .takes_value(true)
            .help("Width of the page. Optional. Default: 800"))
        .get_matches();

    let input_file = matches.value_of("path").unwrap();
    let mut command = vec!["mmdc", "-i", input_file];

    if let Some(output_file) = matches.value_of("output") {
        command.push("-o");
        command.push(output_file);
    };
    if let Some(height) = matches.value_of("height") {
        command.push("-H");
        command.push(height);
    };
    if let Some(width) = matches.value_of("width") {
        command.push("-w");
        command.push(width);
    };
    let mut mmdc = Popen::create(&command, PopenConfig::default())?;
        // stdout: Redirection::File(std::io::stdout()),
        // stderr: Redirection::File(std::io::stderr()),
    mmdc.wait();
    Ok(())
}
