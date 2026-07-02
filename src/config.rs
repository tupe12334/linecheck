use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub pattern: String,
    pub warn: Option<usize>,
    pub error: Option<usize>,
}

pub fn load_config(path: &Path) -> Config {
    let Ok(content) = fs::read_to_string(path) else {
        return Config::default();
    };
    serde_yaml::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Warning: failed to parse config {}: {}", path.display(), e);
        Config::default()
    })
}
