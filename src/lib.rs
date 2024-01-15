mod extractor;

#[cfg(target_os="macos")]
mod egui_app;
#[cfg(target_os="macos")]
pub use egui_app::ExtractorApp;

#[cfg(target_os="windows")]
mod nwm_app;
#[cfg(target_os="windows")]
pub use nwm_app::NwmApp;

