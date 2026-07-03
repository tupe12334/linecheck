pub mod checker;
pub mod config;
pub mod display;
pub mod files;
pub mod lines;

pub use checker::{check_file, FileResult, Status};
pub use config::{load_config, Config, Rule};
pub use files::collect_files;
