use clap::{Parser, Subcommand};
use log::LevelFilter;
use madara_cli::app::config::RollupMode;
use madara_cli::cli;
use madara_cli::cli::explorer::ExplorerOpts;
use madara_cli::da::da_layers::DALayer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Init a new App Chain config
    Init {
        /// App chain name
        #[clap(short, long = "chain-name")]
        name: Option<String>,
        /// Choose a supported Rollup Mode
        #[clap(short, long = "chain-mode", value_enum, ignore_case = true)]
        mode: Option<RollupMode>,
        /// Choose a supported DA Layer
        #[clap(short, long = "da-layer", value_enum, ignore_case = true)]
        da: Option<DALayer>,
    },
    /// Lists all the existing App Chain configs
    List,
    /// Runs the App Chain using Madara
    Run {
        /// App chain name
        #[clap(short, long = "chain-name")]
        name: Option<String>,
        /// Additional arguments for Madara
        madara_flags: Vec<String>,
    },
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
        Some(Commands::Init { name, mode, da }) => cli::init::init(name, mode, da).await,
        Some(Commands::List) => cli::list::list(),
        Some(Commands::Run { name, madara_flags}) => cli::run::run(name, madara_flags).await,
        Some(Commands::Explorer(opts)) => cli::explorer::explorer(opts).await,
        None => log::info!("Use --help to see the complete list of available commands"),
    }
}
