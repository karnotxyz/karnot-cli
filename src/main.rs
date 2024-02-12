use clap::{Parser, Subcommand};
use log::LevelFilter;
use madara_cli::cli;
use madara_cli::cli::explorer::ExplorerOpts;
use madara_cli::cli::run::RunOpts;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    /// Init a new App Chain config
    Init,
    /// Lists all the existing App Chain configs
    List,
    /// Runs the App Chain using Madara
    Run(RunOpts),
    /// Runs the L2 explorer
    Explorer(ExplorerOpts),
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .format_timestamp(None)
        .format_level(false)
        .format_target(false)
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => cli::init::init().await,
        Some(Commands::List) => cli::list::list(),
        Some(Commands::Run(opts)) => cli::run::run(opts).await,
        Some(Commands::Explorer(opts)) => cli::explorer::explorer(opts).await,
        None => log::info!("Use --help to see the complete list of available commands"),
    }
}
