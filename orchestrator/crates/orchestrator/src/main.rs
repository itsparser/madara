/// Contains the CLI arguments for the service
pub mod args;
/// Client for the Orchestrator
// pub mod client;

/// Contains the CLI arguments for the service
pub mod constants;
/// Contain the controllers for the service
pub mod controller;

/// Contains the core logic for the service
pub mod core;

// pub mod config;
/// contains all the error handling / errors that can be returned by the service
pub mod error;
/// contains all the resources that can be used by the service
pub mod resource;
/// Contains all the services that are used by the service
pub mod service;
/// Contains all the utils that are used by the service
pub mod utils;

use crate::args::{Cli, Commands, RunCmd, SetupCmd};
use crate::error::OrchestratorResult;
use crate::resource::setup::setup;
use crate::utils::logging::init_logging;
use clap::Parser as _;
use dotenvy::dotenv;

#[global_allocator]
static A: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();
    init_logging();
    tracing::info!("Starting orchestrator");

    match &cli.command {
        Commands::Run { run_command } => {
            if let Err(e) = run_orchestrator(run_command).await {
                tracing::error!("Failed to run orchestrator: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Setup { setup_command } => {
            if let Err(e) = setup_orchestrator(setup_command).await {
                tracing::error!("Failed to setup orchestrator: {}", e);
                std::process::exit(1);
            }
        }
    }
}

async fn run_orchestrator(run_cmd: &RunCmd) -> OrchestratorResult<()> {
    Ok(())
}

async fn setup_orchestrator(setup_cmd: &SetupCmd) -> OrchestratorResult<()> {
    setup(setup_cmd).await
}
