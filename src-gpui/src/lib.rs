// GPUI AutoEQ Library
//
// This library provides reusable components and workflows for building
// audio equalization interfaces using the GPUI framework.
#![recursion_limit = "256"]

pub mod app_setup;
pub mod app_theme;
pub mod components;
pub mod design;
pub mod workflows;

// Re-export commonly used types from the backend
pub use autoeq_backend::audio::{get_audio_devices, AudioDevice as AudioDeviceInfo};
pub use autoeq_backend::export::FilterParam as ExportFilterParam;
pub use autoeq_backend::optim::run_optimization_internal as run_optimization;
pub use autoeq_backend::{CurveData, OptimizationParams, OptimizationResult};

// Re-export app setup for convenience
pub use app_setup::{init_app, setup_app, setup_keybindings, setup_menu, Quit};
