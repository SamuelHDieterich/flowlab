//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
use flowlab::*;

// Built-in modules
//// CLI argument parser
use clap::Parser;
use tracing_appender::non_blocking::WorkerGuard;
//// Basic data structures
use std::{fmt::Debug, path::PathBuf};

// External crates
//// Logging framework
use tracing_subscriber::{
    fmt::{format::FmtSpan, Subscriber},
    layer::SubscriberExt,
    EnvFilter, Layer,
};

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

//-------------------//
//---  FUNCTIONS  ---//
//-------------------//

/// String to tracing log level parser
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

fn setup_logging(
    // Directory to store the log files
    directory: PathBuf,
    // Base name of the log files, without the extension
    file_name: PathBuf,
    // Log level used for the STDOUT stream
    stdout_log_level: tracing::Level,
) -> Result<(WorkerGuard, WorkerGuard), Box<dyn std::error::Error>> {
    let mut log_file_name = file_name.clone();
    let mut jsonl_file_name = file_name;

    log_file_name.set_extension("log");
    jsonl_file_name.set_extension("jsonl");

    let log_file_appender = tracing_appender::rolling::never(directory.clone(), log_file_name);
    let (log_non_blocking, _log_guard) = tracing_appender::non_blocking(log_file_appender);

    let jsonl_file_appender = tracing_appender::rolling::never(directory, jsonl_file_name);
    let (jsonl_non_blocking, _jsonl_guard) = tracing_appender::non_blocking(jsonl_file_appender);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_line_number(false)
        .with_target(false)
        .with_file(false)
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::from_default_env().add_directive(stdout_log_level.into()));

    let log_file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .pretty()
        .with_line_number(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_file(true)
        .with_writer(log_non_blocking)
        .with_filter(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()));

    let jsonl_file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_line_number(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_file(true)
        .json()
        .with_writer(jsonl_non_blocking)
        .with_filter(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()));

    let registry = tracing_subscriber::registry()
        .with(stdout_layer)
        .with(log_file_layer)
        .with(jsonl_file_layer);

    tracing::subscriber::set_global_default(registry)?;

    Ok((_log_guard, _jsonl_guard))
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

    // Setup logging
    let _guard = setup_logging(
        PathBuf::from("logs"),
        PathBuf::from(format!(
            "flowlab-{}",
            chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
        )),
        cli.log_level,
    )?;

    // Test pipeline deserialization
    tracing::info!("Loading pipeline");
    let pipeline: pipeline::Pipeline<device::Protocols> =
        serde_yaml::from_reader(std::fs::File::open(cli.pipeline)?)?;
    tracing::info!("Pipeline loaded");
    // println!("{:#?}", pipeline);

    Ok(())
}
