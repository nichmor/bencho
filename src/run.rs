use std::path::{Path, PathBuf};

// use miette::{IntoDiagnostic, Result};
use anyhow::{Context, Result};
use clap::{ArgAction, Parser};
use resvg::usvg_text_layout::{fontdb, TreeTextToPath};

use hyperfine_lib::{
    benchmark::{benchmark_result::BenchmarkResult, scheduler},
    export::ExportManager,
    options::Options,
};

use hyperfine_lib::command::{Command as BenchCommand, Commands as BenchCommands};

#[derive(Parser, Debug, Default)]
#[clap(arg_required_else_help = true)]
pub struct Args {
    /// lists of commands to benchmark
    #[arg(required = true, num_args=2..)]
    args: Vec<String>,
    /// Perform NUM warmup runs before the actual benchmark.
    /// This can be used to fill (disk) caches for I/O-heavy programs.
    #[arg(long, short)]
    warmup: Option<u64>,

    /// Execute CMD before each timing run. This is useful for
    /// clearing disk caches, for example. The --prepare option can
    /// be specified once for all commands or multiple times, once for
    // each command. In the latter case, each preparation command will
    // be run prior to the corresponding benchmark command.
    #[arg(long, short, num_args=1, action=ArgAction::Append)]
    prepare: Option<Vec<String>>,

    /// Execute CMD after the completion of all benchmarking
    /// runs for each individual command to be benchmarked.
    /// This is useful if the commands to be benchmarked produce
    /// artifacts that need to be cleaned up.
    #[arg(long, short)]
    cleanup: Option<String>,
}

pub fn run(args: Args) -> Result<()> {
    let fontdb = load_fonts();

    let root = PathBuf::from(std::file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();

    let mut _commands = Vec::default();
    for arg in &args.args {
        let command = BenchCommand::new(Some(arg), arg.as_str());
        _commands.push(command);
    }

    let exp_manager = ExportManager::default();

    let commands = BenchCommands::from(_commands);

    let options = Options {
        cleanup_command: args.cleanup,
        preparation_command: args.prepare,
        warmup_count: args.warmup.unwrap_or(0),

        ..Default::default()
    };

    let mut scheduler = scheduler::Scheduler::new(&commands, &options, &exp_manager);

    scheduler.run_benchmarks()?;

    render_to_png(
        &plot_benchmark("Program Run Comparison", &scheduler.results)?,
        &root.join("assets").join("benchmarks-plot.png"),
        &fontdb,
    )?;

    Ok(())
}

fn plot_benchmark(heading: &str, results: &Vec<BenchmarkResult>) -> Result<String> {
    let mut data = Vec::new();
    for result in results {
        data.push((result.mean, &result.command));
    }

    poloto::build::bar::gen_simple("", data, [0.0])
        .label((heading, "Time (s)", "Command"))
        .append_to(poloto::header().light_theme())
        .render_string()
        .context("Can't render heading table")
}

fn render_to_png(data: &str, path: &Path, fontdb: &fontdb::Database) -> Result<()> {
    let mut tree = resvg::usvg::Tree::from_str(data, &Default::default())?;
    tree.convert_text(fontdb);
    let fit_to = resvg::usvg::FitTo::Width(1600);
    let size = fit_to
        .fit_to(tree.size.to_screen_size())
        .ok_or_else(|| anyhow::anyhow!("failed to fit to screen size"))?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(size.width(), size.height()).unwrap();
    resvg::render(
        &tree,
        fit_to,
        resvg::tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .ok_or_else(|| anyhow::anyhow!("failed to render"))?;
    std::fs::create_dir_all(path.parent().unwrap())?;
    pixmap.save_png(path)?;
    Ok(())
}

fn load_fonts() -> fontdb::Database {
    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();
    fontdb.set_serif_family("Times New Roman");
    fontdb.set_sans_serif_family("Arial");
    fontdb.set_cursive_family("Comic Sans MS");
    fontdb.set_fantasy_family("Impact");
    fontdb.set_monospace_family("Courier New");

    fontdb
}
