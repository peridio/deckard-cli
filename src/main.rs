use clap::Parser;
use tracing::{debug, info};

mod args;
mod commands;
mod constants;
mod error;
mod html;
mod json_schema;

use args::{effective_log_level, GlobalArgs};
use commands::Commands;
use error::Result;

#[derive(Parser, Debug)]
#[command(name = "deckard")]
#[command(about = "A Rust CLI application template")]
#[command(version)]
struct Cli {
    #[command(flatten)]
    global: GlobalArgs,

    #[command(subcommand)]
    command: Commands,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("\033[1;31m[ERROR]\033[0m {}", e);

        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing based on effective log level
    let log_level = effective_log_level(&cli.global);
    init_tracing(log_level);

    debug!("CLI arguments: {:?}", cli);
    info!("Starting command execution.");

    match cli.command {
        Commands::Upgrade(args) => commands::upgrade::execute(args),
        Commands::Convert(args) => commands::convert::execute(args),
    }
}

fn init_tracing(log_level: args::LogLevel) {
    let filter = log_level.as_filter();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(filter)),
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_writer(std::io::stderr)
        .compact()
        .init();

    debug!("Logging initialized at level: {}", log_level);
}
