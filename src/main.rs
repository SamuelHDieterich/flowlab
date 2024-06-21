//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
use flowlab::*;

// Built-in modules
//// CLI argument parser
use clap::Parser;
//// Basic data structures
use std::path::PathBuf;

// External crates
//// Logging framework
use tracing_subscriber::fmt::{format::FmtSpan, Subscriber};

//-----------------//
//---  STRUCTS  ---//
//-----------------//

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Pipeline configuration file
    #[arg(short, long, value_name = "FILE", value_hint = clap::ValueHint::FilePath, required = true)]
    pipeline: PathBuf,
    /// Log level
    #[arg(
        short,
        long,
        value_name = "LEVEL",
        value_parser(log_level_parser),
        default_value = "info"
    )]
    log_level: tracing::Level,
}

fn log_level_parser(s: &str) -> Result<tracing::Level, String> {
    match s.to_lowercase().as_str() {
        "error" => Ok(tracing::Level::ERROR),
        "warn" => Ok(tracing::Level::WARN),
        "info" => Ok(tracing::Level::INFO),
        "debug" => Ok(tracing::Level::DEBUG),
        "trace" => Ok(tracing::Level::TRACE),
        _ => Err("Invalid log level".to_string()),
    }
}

//--------------//
//---  MAIN  ---//
//--------------//

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read command-line arguments
    tracing::info!("Starting FlowLab");
    let cli = Cli::parse();
    tracing::debug!(?cli);

    // Setup tracing subscriber - Logging system
    Subscriber::builder()
        .with_max_level(cli.log_level)
        .pretty()
        .with_line_number(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_file(true)
        .init();

    // Test pipeline deserialization
    tracing::info!("Loading pipeline");
    let _pipeline: pipeline::Pipeline<device::Protocols> =
        serde_yaml::from_reader(std::fs::File::open(cli.pipeline)?)?;
    tracing::info!("Pipeline loaded");
    println!("{:#?}", _pipeline);

    Ok(())
}
