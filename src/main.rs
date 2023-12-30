use clap::{Parser, Subcommand};
use karnot_cli::cli;

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
    Run
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => cli::init::init(),
        Some(Commands::List) => cli::list::list(),
        Some(Commands::Run) => cli::run::run(),
        None => log::info!("Use --help to see the complete list of available commands"),
    }
}
