mod run;

use clap::{Parser, Subcommand};

use anyhow::Result;

use run::run;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the benchmark
    Run(run::Args),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => run(args)?,
    }

    Ok(())
}
