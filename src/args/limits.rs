use super::Args;
use linecheck::preset::{DEFAULT_ERROR, DEFAULT_WARN, Preset};

impl Args {
    /// Resolve the selected preset (if any) into fallback warn/error limits.
    pub fn fallback_limits(&self) -> (Option<usize>, Option<usize>) {
        let preset = self
            .strict
            .then_some(Preset::Strict)
            .or(self.default_preset.then_some(Preset::Default))
            .or(self.loose.then_some(Preset::Loose))
            .or(self.free.then_some(Preset::Free));
        preset
            .map(Preset::limits)
            .unwrap_or((Some(DEFAULT_WARN), Some(DEFAULT_ERROR)))
    }
}
