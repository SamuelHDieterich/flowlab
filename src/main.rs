use clap::Parser;
use flowlab::{device, instruction, parser};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing_subscriber::fmt;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Device configuration file(s)/folder(s)
    #[arg(short, long, value_name = "FILE/FOLDER", value_hint = clap::ValueHint::FilePath, required = true)]
    device: Vec<PathBuf>,
    /// Instruction configuration file(s)/folder(s)
    #[arg(short, long, value_name = "FILE/FOLDER", value_hint = clap::ValueHint::FilePath, required = true)]
    instruction: Vec<PathBuf>,
    /// Pipeline configuration file
    #[arg(short, long, value_name = "FILE", value_hint = clap::ValueHint::FilePath, required = true)]
    pipeline: PathBuf,
}

#[tokio::main]
async fn main() {
    fmt::Subscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();

    let devices: Vec<device::Device<device::Protocols>> = parser::parse_files(cli.device).await;
    for device in &devices {
        println!("{}\n", device);
    }

    let instructions: HashMap<String, Vec<instruction::DeviceCommand>> =
        parser::parse_files_with_filename(cli.instruction).await;
    for (filename, instruction) in &instructions {
        println!("Filename: {}", filename);
        for i in instruction {
            println!("{}", i);
        }
    }

    let pipeline: Vec<instruction::PipelineStep> = parser::parse(cli.pipeline).await.unwrap();
    for p in &pipeline {
        println!("{}", p);
    }
}
