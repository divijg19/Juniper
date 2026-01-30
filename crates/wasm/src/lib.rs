//! Minimal wasm plugin host API (stubbed).
//!
//! Real plugin execution must be sandboxed (WASM + capability-based IO). This
//! module provides a tiny trait to represent plugin behaviour during prototyping.

/// Result returned by a plugin invocation.
#[derive(Debug, Clone)]
pub struct PluginResult {
    pub output: String,
}

/// Lightweight trait that a plugin should implement. Real hosts will
/// instantiate plugins inside WASM VMs and translate calls across the boundary.
pub trait Plugin {
    fn run(&self, input: &str) -> PluginResult;
}

/// Library version.
pub fn version() -> &'static str {
    "0.1.0"
}
