/// Built-in strictness presets applied when no config rule matches a file.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Preset { Strict, Default, Loose, Free }

/// Default limits applied when no config is found anywhere.
pub const DEFAULT_WARN: usize = 200;
pub const DEFAULT_ERROR: usize = 400;

impl Preset {
    /// Returns (warn_limit, error_limit); None means unlimited.
    pub fn limits(self) -> (Option<usize>, Option<usize>) {
        match self {
            Preset::Strict  => (Some(100), Some(100)),
            Preset::Default => (Some(DEFAULT_WARN), Some(DEFAULT_ERROR)),
            Preset::Loose   => (Some(400), Some(400)),
            Preset::Free    => (None, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preset_limits() {
        assert_eq!(Preset::Strict.limits(), (Some(100), Some(100)));
        assert_eq!(Preset::Free.limits(), (None, None));
        assert_eq!(Preset::Default.limits(), (Some(DEFAULT_WARN), Some(DEFAULT_ERROR)));
    }
}
