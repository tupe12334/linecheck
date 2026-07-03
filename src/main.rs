use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use linecheck::config::load_config;
use linecheck::display::{print_status, print_violations};
use linecheck::files::collect_files;

#[derive(Parser, Debug)]
#[command(name = "linecheck", about = "Warn or error when files exceed a set line count")]
struct Args {
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,
    #[arg(long)]
    max_lines: Option<usize>,
    #[arg(long, default_value = "linecheck.yml")]
    config: PathBuf,
    #[arg(long, help = "Show line count status for all files, including those within limits")]
    status: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = load_config(&args.config);
    let files = collect_files(&args.paths, &config.exclude);
    let mut has_error = false;
    if args.status {
        print_status(&files, &config, args.max_lines, &mut has_error)?;
    } else {
        print_violations(&files, &config, args.max_lines, &mut has_error)?;
    }
    std::process::exit(if has_error { 1 } else { 0 });
}
