use super::Args;

impl Args {
    pub fn config_path(&self) -> Option<std::path::PathBuf> {
        let c = &self.config;
        if c.strip_prefix(".").unwrap_or(c) != std::path::Path::new("linecheck.yml") {
            if !c.exists() {
                eprintln!("Error: config file '{}' not found", c.display());
                std::process::exit(1);
            }
            return Some(c.clone());
        }
        None
    }
}
