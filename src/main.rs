mod args;
mod display;

use anyhow::Result;
use args::Args;
use clap::Parser;
use display::{print_json, print_status, print_violations};
use linecheck::{checker::CheckOptions, config::ConfigResolver, files::collect_files};

fn main() -> Result<()> {
    let args = Args::parse();
    let mut resolver = ConfigResolver::new(args.config_path(), "linecheck.yml");
    let (fallback_warn, fallback_error) = args.fallback_limits();
    let opts = CheckOptions {
        max_lines: args.max_lines,
        fallback_warn,
        fallback_error,
        skip_comments: args.skip_comments,
    };
    let root_cfg = resolver.resolve(&std::env::current_dir().unwrap_or_default().join("__"));
    let files = collect_files(
        &args.paths,
        &root_cfg
            .as_ref()
            .map(|c| c.exclude.clone())
            .unwrap_or_default(),
    );
    let mut has_error = false;
    if args.json {
        print_json(&files, &mut resolver, &opts, args.status, &mut has_error);
    } else if args.status {
        print_status(&files, &mut resolver, &opts, &mut has_error);
    } else {
        print_violations(&files, &mut resolver, &opts, &mut has_error);
    }
    if has_error {
        std::process::exit(1);
    }
    Ok(())
}
