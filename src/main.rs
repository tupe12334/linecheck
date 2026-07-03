mod checker;
mod config;
mod files;
mod lines;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use checker::{Status, check_file};
use config::load_config;
use files::collect_files;

#[derive(Parser, Debug)]
#[command(name = "linecheck", about = "Warn or error when files exceed a set line count")]
struct Args {
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,
    #[arg(long)]
    max_lines: Option<usize>,
    #[arg(long, default_value = "linecheck.yml")]
    config: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = load_config(&args.config);
    let files = collect_files(&args.paths, &config.exclude);
    let mut has_error = false;
    for file in &files {
        match check_file(file, &config, args.max_lines) {
            Ok((status, lines, threshold)) if status >= Status::Warn => {
                let kind = if status == Status::Error { "error" } else { "warn" };
                let limit = threshold.map_or(String::new(), |t| format!(" ({kind} threshold: {t})"));
                println!("{}: {lines} lines{limit}", file.display());
                if status == Status::Error { has_error = true; }
            }
            Err(e) => eprintln!("Error: {}", e),
            _ => {}
        }
    }
    std::process::exit(if has_error { 1 } else { 0 });
}
