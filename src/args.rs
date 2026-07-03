use clap::{ArgGroup, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "linecheck", about = "Warn or error when files exceed a set line count", version)]
#[command(group(ArgGroup::new("preset_group").args(["strict", "default_preset", "loose", "free"])))]
pub struct Args {
    #[arg(default_value = ".", help = "Files or directories to check")]
    pub paths: Vec<PathBuf>,
    #[arg(long, help = "Override line limit for all files")]
    pub max_lines: Option<usize>,
    #[arg(long, default_value = "linecheck.yml", help = "Path to config file")]
    pub config: PathBuf,
    #[arg(long, help = "Show all files with their line counts and usage percentage")]
    pub status: bool,
    #[arg(long, help = "Output results as JSON")]
    pub json: bool,
    #[arg(long, help = "Preset: 100 lines (warn=error)")]
    pub strict: bool,
    #[arg(long = "default", help = "Preset: 200/400 lines (warn/error)")]
    pub default_preset: bool,
    #[arg(long, help = "Preset: 400 lines (warn=error)")]
    pub loose: bool,
    #[arg(long, help = "Preset: unlimited (disable all limits)")]
    pub free: bool,
}
