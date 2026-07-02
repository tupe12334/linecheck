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
    #[arg(long, default_value = ".linecheckrc")]
    config: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = load_config(&args.config);
    let files = collect_files(&args.paths);
    let mut worst = Status::Ok;
    for file in &files {
        match check_file(file, &config, args.max_lines) {
            Ok((status, lines)) if status >= Status::Warn => {
                let label = if status == Status::Error { "ERROR" } else { "WARN" };
                println!("{} {} ({} lines)", label, file.display(), lines);
                if status > worst { worst = status; }
            }
            Err(e) => eprintln!("Error: {}", e),
            _ => {}
        }
    }
    std::process::exit(match worst {
        Status::Ok => 0,
        Status::Warn => 1,
        Status::Error => 2,
    });
}
