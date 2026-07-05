//! YAML configuration loading and hierarchical config resolution.
mod resolver;
mod validate;
use crate::rule::Rule;
pub use resolver::ConfigResolver;
use serde::Deserialize;
use std::fs;
use std::path::Path;
pub(crate) use validate::warn_invalid_patterns;

/// A parsed `linecheck.yml` configuration file.
#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct Config {
    /// Ordered list of glob rules. The first matching rule wins.
    #[serde(default)]
    pub rules: Vec<Rule>,
    /// Glob patterns for files and directories to skip entirely.
    #[serde(default)]
    pub exclude: Vec<String>,
}

/// Load a `linecheck.yml` from `path`. Returns [`Config::default`] on any error.
#[must_use]
pub fn load_config(path: &Path) -> Config {
    let Ok(s) = fs::read_to_string(path) else {
        return Config::default();
    };
    let cfg: Config = serde_yaml::from_str(&s).unwrap_or_else(|e| {
        eprintln!("Warning: failed to parse config {}: {}", path.display(), e);
        Config::default()
    });
    warn_invalid_patterns(&cfg, path);
    cfg
}

#[cfg(test)]
#[path = "../config_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../config_tests_2.rs"]
mod tests2;
