use clap::{Command, Arg};
use subprocess::{Popen, PopenConfig, Redirection};

fn main() -> Result<(), subprocess::PopenError> {
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
            .required(true)
            .takes_value(true)
            .help("Name of the output file"))
        .arg(Arg::new("resolution")
            .short('r')
            .long("resolution")
            .required(false)
            .takes_value(true)
            .help("Resolution of the output file"))
        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .required(false)
            .takes_value(true)
            .help("Format of the output, should be md, svg, png or pdf"))
        .get_matches();
    
    /*println!("{}\n{}\n{}\n{}", 
        matches.value_of("output").unwrap(), 
        matches.value_of("path").unwrap(),
        matches.value_of("resolution").unwrap_or("NO_RESOLUTION"),
        matches.value_of("format").unwrap_or("NO_FORMAT"),
    );*/

    let input_file = matches.value_of("path").unwrap();
    let output_file = matches.value_of("output").unwrap();

    let mut mmdc = Popen::create(&["mmdc", "-i", input_file, "-o", output_file], PopenConfig::default())?;
        // stdout: Redirection::File(std::io::stdout()),
        // stderr: Redirection::File(std::io::stderr()),
    mmdc.wait();
    Ok(())
}
