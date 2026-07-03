//! YAML configuration loading and hierarchical config resolution.
use crate::rule::Rule;
use glob::Pattern;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
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
fn warn_invalid_patterns(cfg: &Config, source: &Path) {
    let pairs = cfg
        .rules
        .iter()
        .map(|r| (r.pattern.as_str(), "glob rule"))
        .chain(cfg.exclude.iter().map(|p| (p.as_str(), "exclude")));
    for (pat, kind) in pairs {
        if Pattern::new(pat).is_err() {
            eprintln!(
                "Warning: invalid {kind} pattern {pat:?} in {} — will be skipped",
                source.display()
            );
        }
    }
}
/// Resolves per-file configs by walking up the directory tree, caching results.
pub struct ConfigResolver {
    explicit: Option<PathBuf>,
    config_name: String,
    cache: HashMap<PathBuf, Option<Config>>,
}
impl ConfigResolver {
    /// Create a resolver: `Some(path)` pins one config, `None` enables hierarchical lookup.
    pub fn new(explicit: Option<PathBuf>, config_name: &str) -> Self {
        Self {
            explicit,
            config_name: config_name.to_owned(),
            cache: HashMap::new(),
        }
    }
    /// Returns the config for `file`, or `None` if no config file is found.
    pub fn resolve(&mut self, file: &Path) -> Option<Config> {
        if let Some(ref p) = self.explicit.clone() {
            return self.load_cached(p.clone());
        }
        self.load_cached(
            file.parent()?
                .ancestors()
                .map(|a| a.join(&self.config_name))
                .find(|c| c.exists())?,
        )
    }
    fn load_cached(&mut self, path: PathBuf) -> Option<Config> {
        self.cache
            .entry(path.clone())
            .or_insert_with(|| {
                fs::read_to_string(&path).ok().and_then(|s| {
                    serde_yaml::from_str::<Config>(&s)
                        .ok()
                        .inspect(|c| warn_invalid_patterns(c, &path))
                })
            })
            .clone()
    }
}
