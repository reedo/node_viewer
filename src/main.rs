// Hide the console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use node_viewer::init::start_app;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
    start_app()
}

#[cfg(target_arch = "wasm32")]
fn main() -> anyhow::Result<()> {
    start_app()
}
