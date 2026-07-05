//! WASM bindings for `linecheck`, published to npm as `linecheck`.
//!
//! This crate is a thin wrapper: all checking logic lives in the `linecheck`
//! core crate's [`linecheck::check_content`], which works on in-memory bytes
//! rather than reading from disk. That's what makes this binding possible in a
//! sandboxed WASM runtime with no filesystem access — and it's the same entry
//! point a future Go binding (e.g. via a WASI build consumed by wazero/wasmtime-go,
//! since `wasm-bindgen`'s JS glue isn't usable from Go) would reuse instead of
//! reimplementing the check logic.
mod result;

use linecheck::{CheckOptions, Config, check_content};
use result::CheckResult;
use std::path::Path;
use wasm_bindgen::prelude::*;

/// Check in-memory file content against a `linecheck.yml`-style config.
///
/// - `filename`: virtual path used only for glob-pattern matching (e.g. `"src/main.rs"`);
///   it does not need to exist anywhere.
/// - `content`: the file's contents.
/// - `config_yaml`: optional `linecheck.yml` source; omit (`undefined`/`null`) to fall
///   back to the built-in 200/400 warn/error thresholds.
#[wasm_bindgen]
pub fn check(filename: &str, content: &str, config_yaml: Option<String>) -> Result<JsValue, JsValue> {
    let config = config_yaml
        .map(|yaml| serde_yaml::from_str::<Config>(&yaml))
        .transpose()
        .map_err(|e| JsValue::from_str(&format!("invalid linecheck config: {e}")))?;

    let result = check_content(
        Path::new(filename),
        content.as_bytes(),
        config.as_ref(),
        &CheckOptions::default(),
    );

    let out = CheckResult::from(result);
    serde_wasm_bindgen::to_value(&out).map_err(|e| JsValue::from_str(&e.to_string()))
}
