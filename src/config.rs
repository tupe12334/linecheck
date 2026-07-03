//! YAML configuration loading and hierarchical resolution.
use glob::Pattern;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// A parsed `linecheck.yml` configuration file.
#[derive(Debug, Deserialize, Default, Clone, PartialEq)]
pub struct Config {
    /// Ordered list of glob rules. The first matching rule wins.
    #[serde(default)]
    pub rules: Vec<Rule>,
    /// Glob patterns for files and directories to skip entirely.
    #[serde(default)]
    pub exclude: Vec<String>,
}

/// A single pattern/limit pair inside a [`Config`].
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Rule {
    /// Glob pattern matched against file paths (e.g. `"**/*.rs"`).
    pub pattern: String,
    /// Line count at which a warning is emitted. `None` means no warn limit.
    pub warn: Option<usize>,
    /// Line count at which an error is emitted. `None` means no error limit.
    pub error: Option<usize>,
}

/// Load a `linecheck.yml` from `path`. Returns [`Config::default`] on any error.
pub fn load_config(path: &Path) -> Config {
    let Ok(content) = fs::read_to_string(path) else {
        return Config::default();
    };
    let cfg: Config = serde_yaml::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Warning: failed to parse config {}: {}", path.display(), e);
        Config::default()
    });
    warn_invalid_patterns(&cfg, path);
    cfg
}

fn warn_invalid_patterns(cfg: &Config, source: &Path) {
    let check = |pat: &str, kind: &str| {
        if Pattern::new(pat).is_err() {
            eprintln!("Warning: invalid {kind} pattern {pat:?} in {} — will be skipped", source.display());
        }
    };
    for rule in &cfg.rules { check(&rule.pattern, "glob rule"); }
    for pat in &cfg.exclude { check(pat, "exclude"); }
}

/// Resolves per-file configs by walking up the directory tree,
/// caching loaded configs to avoid redundant disk reads.
pub struct ConfigResolver {
    explicit: Option<PathBuf>,
    config_name: String,
    cache: HashMap<PathBuf, Option<Config>>,
}

impl ConfigResolver {
    /// Create a resolver. Pass `explicit` when the user supplied `--config`;
    /// pass `None` to enable automatic hierarchical lookup.
    pub fn new(explicit: Option<PathBuf>, config_name: &str) -> Self {
        Self { explicit, config_name: config_name.to_owned(), cache: HashMap::new() }
    }

    /// Returns the config that applies to `file`.
    ///
    /// If an explicit config was provided on the CLI, always returns that.
    /// Otherwise walks up the directory tree to find the nearest `linecheck.yml`.
    pub fn resolve(&mut self, file: &Path) -> Option<Config> {
        if let Some(ref p) = self.explicit.clone() {
            return self.load_cached(p.clone());
        }
        let dir = file.parent()?;
        for ancestor in dir.ancestors() {
            let candidate = ancestor.join(&self.config_name);
            if candidate.exists() {
                return self.load_cached(candidate);
            }
        }
        None
    }

    fn load_cached(&mut self, path: PathBuf) -> Option<Config> {
        self.cache
            .entry(path.clone())
            .or_insert_with(|| {
                fs::read_to_string(&path).ok().and_then(|s| {
                    serde_yaml::from_str::<Config>(&s).ok().inspect(|cfg| {
                        warn_invalid_patterns(cfg, &path);
                    })
                })
            })
            .clone()
    }
}
