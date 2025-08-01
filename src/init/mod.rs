#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(not(target_arch = "wasm32"))]
pub use native::start_app;

#[cfg(target_arch = "wasm32")]
pub use web::start_app;
