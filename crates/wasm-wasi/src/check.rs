use crate::memory::{leak, read_bytes, read_str};
use crate::result::CheckResult;
use linecheck::{CheckOptions, Config, check_content};
use std::path::Path;

/// Check in-memory file content against a `linecheck.yml`-style config.
///
/// `filename`/`content`/`config_yaml` are read as UTF-8 byte ranges out of linear
/// memory (`config_len == 0` means "no config, use built-in 200/400 warn/error
/// thresholds"). Returns a packed `(ptr << 32) | len` pointing at a JSON-encoded
/// result object (`{status, lines, warn_limit, error_limit, message}`); a malformed
/// config yields `status: "error"` with the parse error in `message`. The caller
/// must free the returned pointer with [`crate::memory::dealloc`].
///
/// # Safety
/// Each `_ptr`/`_len` pair must describe a valid, initialized, readable UTF-8 byte
/// range in this module's linear memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check(
    filename_ptr: u32,
    filename_len: u32,
    content_ptr: u32,
    content_len: u32,
    config_ptr: u32,
    config_len: u32,
) -> u64 {
    let filename = unsafe { read_str(filename_ptr, filename_len) };
    let content = unsafe { read_bytes(content_ptr, content_len) };
    let config_yaml = (config_len > 0).then(|| unsafe { read_str(config_ptr, config_len) });

    let out = match config_yaml.map(serde_yaml::from_str::<Config>) {
        Some(Err(e)) => CheckResult::config_error(e.to_string()),
        Some(Ok(config)) => CheckResult::from(check_content(
            Path::new(filename),
            content,
            Some(&config),
            &CheckOptions::default(),
        )),
        None => CheckResult::from(check_content(
            Path::new(filename),
            content,
            None,
            &CheckOptions::default(),
        )),
    };

    leak(serde_json::to_vec(&out).unwrap_or_default().into_boxed_slice())
}
