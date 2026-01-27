/// Minimal wasm plugin host API (stubbed). Real implementation should sandbox execution.
#[derive(Debug, Clone)]
pub struct PluginResult {
    pub output: String,
}

pub trait Plugin {
    fn run(&self, input: &str) -> PluginResult;
}

pub fn version() -> &'static str {
    "0.1.0"
}
