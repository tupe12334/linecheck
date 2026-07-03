use super::*;

#[test]
fn preset_strict_limits() {
    assert_eq!(Preset::Strict.limits(), (Some(100), Some(100)));
}

#[test]
fn preset_default_limits() {
    assert_eq!(Preset::Default.limits(), (Some(DEFAULT_WARN), Some(DEFAULT_ERROR)));
}

#[test]
fn preset_loose_limits() {
    assert_eq!(Preset::Loose.limits(), (Some(400), Some(400)));
}

#[test]
fn preset_free_limits() {
    assert_eq!(Preset::Free.limits(), (None, None));
}
