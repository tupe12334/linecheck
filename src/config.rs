//! YAML configuration loading and hierarchical resolution.
use crate::rule::Rule;
use glob::Pattern;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::{Path, PathBuf}};

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
pub fn load_config(path: &Path) -> Config {
    let Ok(s) = fs::read_to_string(path) else { return Config::default(); };
    let cfg: Config = serde_yaml::from_str(&s).unwrap_or_else(|e| { eprintln!("Warning: failed to parse {}: {}", path.display(), e); Config::default() });
    warn_invalid_patterns(&cfg, path); cfg
}
fn warn_invalid_patterns(cfg: &Config, src: &Path) {
    let check = |p: &str, k: &str| { if Pattern::new(p).is_err() { eprintln!("Warning: invalid {k} pattern {p:?} in {} — will be skipped", src.display()); } };
    for r in &cfg.rules { check(&r.pattern, "glob rule"); }
    for p in &cfg.exclude { check(p, "exclude"); }
}
#[cfg(test)]
#[path = "config_tests.rs"]
mod tests;
/// Resolves per-file configs by walking up the directory tree, caching results.
pub struct ConfigResolver { explicit: Option<PathBuf>, config_name: String, cache: HashMap<PathBuf, Option<Config>> }
impl ConfigResolver {
    /// Create a resolver. `Some(path)` pins one config; `None` enables hierarchical lookup.
    pub fn new(explicit: Option<PathBuf>, config_name: &str) -> Self {
        Self { explicit, config_name: config_name.to_owned(), cache: HashMap::new() }
    }
    /// Returns the config for `file`, or `None` if no config file is found.
    pub fn resolve(&mut self, file: &Path) -> Option<Config> {
        if let Some(ref p) = self.explicit.clone() { return self.load_cached(p.clone()); }
        let dir = file.parent()?;
        for a in dir.ancestors() { let c = a.join(&self.config_name); if c.exists() { return self.load_cached(c); } }
        None
    }
    fn load_cached(&mut self, path: PathBuf) -> Option<Config> {
        self.cache.entry(path.clone()).or_insert_with(|| {
            fs::read_to_string(&path).ok().and_then(|s| serde_yaml::from_str::<Config>(&s).ok().inspect(|c| warn_invalid_patterns(c, &path)))
        }).clone()
    }
}
