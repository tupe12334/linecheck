//! WASI bindings for `linecheck` (not yet published — see the crate's `publish = false`
//! in Cargo.toml), built for the `wasm32-wasip1` target and consumed from Go via
//! wazero or wasmtime-go. Unlike `crates/wasm`, this has no `wasm-bindgen` JS glue,
//! since that glue only targets JS hosts.
//!
//! All checking logic lives in the `linecheck` core crate's
//! [`linecheck::check_content`], which this crate calls the same way `crates/wasm` does.
//!
//! ## Memory ABI
//!
//! The host calls [`memory::alloc`] to get a writable buffer, writes UTF-8 bytes
//! into it, then calls [`check::check`], which returns a packed `(ptr << 32) | len`
//! `u64` pointing at a JSON-encoded result. The host reads that byte range, then
//! calls [`memory::dealloc`] on every pointer/length pair it obtained (its own
//! input buffers and the returned one) to free linear memory. See `examples/go`
//! for a full round trip using wazero.
//!
//! # ponytail: no unit tests here
//! The ABI casts real pointers to `u32`, which only holds on an actual 32-bit
//! `wasm32` target, not a 64-bit test host. `examples/go` (run in CI via wazero
//! against the real `wasm32-wasip1` build) is the runnable check for this module;
//! it asserts the round-trip result, not just prints it.

mod check;
mod memory;
mod result;
