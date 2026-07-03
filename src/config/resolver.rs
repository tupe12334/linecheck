use super::{Config, warn_invalid_patterns};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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
