use clap::Parser;
use flowlab::{device, instruction, pipeline, Mapify};
use std::path::PathBuf;
use tracing_subscriber::fmt;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Instruction configuration file(s)/folder(s)
    #[arg(short, long, value_name = "FILE/FOLDER", value_hint = clap::ValueHint::FilePath, required = true)]
    instruction: Vec<PathBuf>,
    /// Device configuration file(s)/folder(s)
    #[arg(short, long, value_name = "FILE/FOLDER", value_hint = clap::ValueHint::FilePath, required = true)]
    device: Vec<PathBuf>,
    /// Pipeline configuration file
    #[arg(short, long, value_name = "FILE", value_hint = clap::ValueHint::FilePath, required = true)]
    pipeline: PathBuf,
}

fn list_files(filepath: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    fn _list_files(files: &mut Vec<PathBuf>, filepath: &PathBuf) -> Result<(), std::io::Error> {
        if filepath.is_dir() {
            let entries = std::fs::read_dir(filepath)?;
            for entry in entries {
                let path = entry?.path();
                _list_files(files, &path)?;
            }
        } else {
            files.push(filepath.clone());
        }
        Ok(())
    }

    let mut files = Vec::new();
    _list_files(&mut files, filepath)?;
    Ok(files)
}

fn deserialize<T, V>(filepaths: &Vec<PathBuf>) -> Result<T, Box<dyn std::error::Error>>
where
    T: Default + Mapify<PathBuf, V>,
    V: serde::de::DeserializeOwned,
{
    let mut collection = T::default();
    let mut files = Vec::new();
    for filepath in filepaths {
        files.extend(list_files(filepath)?);
    }
    for file in &files {
        let data: V = serde_yaml::from_reader(std::fs::File::open(file)?)?;
        collection.insert(file.clone(), data);
    }
    Ok(collection)
}

#[tokio::main]
async fn main() {
    fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let cli = Cli::parse();

    let instructions: instruction::InstructionCollection = deserialize(&cli.instruction).unwrap();
    println!("{:#?}", instructions);
    println!("{:#?}", instructions.keys());

    let devices: device::DeviceCollection<device::Protocols> = deserialize(&cli.device).unwrap();
    println!("{:#?}", devices);
    println!("{:#?}", devices.keys());

    let pipeline: Vec<pipeline::PipelineStep> =
        serde_yaml::from_reader(std::fs::File::open(cli.pipeline).unwrap()).unwrap();
    println!("{:#?}", pipeline);
}
