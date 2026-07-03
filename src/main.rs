use anyhow::Result;
use clap::{ArgGroup, Parser};
use std::path::PathBuf;

use linecheck::checker::CheckOptions;
use linecheck::config::ConfigResolver;
use linecheck::display::{print_json, print_status, print_violations};
use linecheck::files::collect_files;
use linecheck::preset::Preset;

#[derive(Parser, Debug)]
#[command(name = "linecheck", about = "Warn or error when files exceed a set line count", version)]
#[command(group(ArgGroup::new("preset_group").args(["strict", "default_preset", "loose", "free"])))]
struct Args {
    #[arg(default_value = ".", help = "Files or directories to check")]
    paths: Vec<PathBuf>,
    #[arg(long, help = "Override line limit for all files")]
    max_lines: Option<usize>,
    #[arg(long, default_value = "linecheck.yml", help = "Path to config file")]
    config: PathBuf,
    #[arg(long, help = "Show all files with their line counts and usage percentage")]
    status: bool,
    #[arg(long, help = "Output results as JSON")]
    json: bool,
    #[arg(long, help = "Preset: 100 lines (warn=error)")]
    strict: bool,
    #[arg(long = "default", help = "Preset: 200/400 lines (warn/error)")]
    default_preset: bool,
    #[arg(long, help = "Preset: 400 lines (warn=error)")]
    loose: bool,
    #[arg(long, help = "Preset: unlimited (disable all limits)")]
    free: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let is_default_config_name = args.config.to_str() == Some("linecheck.yml");
    let explicit_config = if !is_default_config_name {
        if !args.config.exists() {
            eprintln!("Error: config file '{}' not found", args.config.display());
            std::process::exit(1);
        }
        Some(args.config.clone())
    } else if args.config.exists() {
        Some(args.config.clone())
    } else {
        None
    };

    let mut resolver = ConfigResolver::new(explicit_config, "linecheck.yml");

    let preset = if args.strict { Some(Preset::Strict) }
        else if args.default_preset { Some(Preset::Default) }
        else if args.loose { Some(Preset::Loose) }
        else if args.free { Some(Preset::Free) }
        else { None };

    let (fallback_warn, fallback_error) = preset
        .map(|p| p.limits())
        .unwrap_or((Some(linecheck::preset::DEFAULT_WARN), Some(linecheck::preset::DEFAULT_ERROR)));

    let opts = CheckOptions { max_lines: args.max_lines, fallback_warn, fallback_error };

    // Collect files using root-level config exclude list (best-effort from explicit config)
    let root_cfg = resolver.resolve(std::env::current_dir().unwrap_or_default().as_path());
    let exclude = root_cfg.as_ref().map(|c| c.exclude.clone()).unwrap_or_default();
    let files = collect_files(&args.paths, &exclude);

    let mut has_error = false;
    if args.json {
        print_json(&files, &mut resolver, &opts, args.status, &mut has_error)?;
    } else if args.status {
        print_status(&files, &mut resolver, &opts, &mut has_error)?;
    } else {
        print_violations(&files, &mut resolver, &opts, &mut has_error)?;
    }
    std::process::exit(if has_error { 1 } else { 0 });
}
