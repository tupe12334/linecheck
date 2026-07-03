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
