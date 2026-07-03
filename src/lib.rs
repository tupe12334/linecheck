//! `linecheck` — enforce per-file line limits across your codebase.
//!
//! This crate is both a CLI tool (`linecheck`) and a Rust library. The library
//! API lets you embed line-count checking into your own tools or test harnesses.
//!
//! # Quick start
//!
//! ```no_run
//! use linecheck::{check_file, CheckOptions};
//! use std::path::Path;
//!
//! let result = check_file(Path::new("src/main.rs"), None, &CheckOptions::default()).unwrap();
//! println!("{} lines — {:?}", result.lines, result.status);
//! ```

pub mod checker;
pub mod config;
pub mod display;
pub mod files;
pub mod lines;
pub mod preset;


pub use checker::{check_file, CheckOptions, FileResult, Status};
pub use config::{load_config, Config, ConfigResolver, Rule};
pub use files::collect_files;
pub use preset::Preset;
