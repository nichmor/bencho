mod run;

use std::{
    path::{Path, PathBuf},
    // process::Command,
    time::Duration,
};

use clap::{ArgAction, Parser, Subcommand};

use anyhow::Result;
use backon::{BlockingRetryable, ConstantBuilder};
use resvg::usvg_text_layout::{fontdb, TreeTextToPath};
use run::{run};
use serde::Deserialize;

use hyperfine_lib::{
    benchmark::{benchmark_result::BenchmarkResult, scheduler},
    export::ExportManager,
    options::{self, Options},
};

use hyperfine_lib::command::{Command as BenchCommand, Commands as BenchCommands};

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
