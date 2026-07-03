#![deny(clippy::all)]
#![deny(missing_docs)]
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
pub mod files;
pub mod lines;
pub mod preset;
pub mod result;
pub mod rule;

pub use checker::{check_file, CheckOptions};
pub use config::{load_config, Config, ConfigResolver};
pub use files::collect_files;
pub use preset::Preset;
pub use result::{FileResult, Status};
pub use rule::Rule;
