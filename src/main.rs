// use flowlab::{device, instruction, parser};
// extern crate clap;
use clap::{Arg, Command};

#[tokio::main]
async fn main() {
    let matches = Command::new("FlowLab")
        .version("0.1.0")
        .about("A monitoring and control system for scientific instruments")
        .arg(
            Arg::new("device")
                .short('d')
                .long("device")
                .value_name("FILE")
                .required(true)
                .help("Device configuration file"),
        )
        .arg(
            Arg::new("instruction")
                .short('i')
                .long("instruction")
                .value_name("FOLDER")
                .required(true)
                .help("Instruction configuration folder"),
        )
        .arg(
            Arg::new("pipeline")
                .short('p')
                .long("pipeline")
                .value_name("FILE")
                .required(true)
                .help("Pipeline configuration file"),
        )
        .get_matches();
}
