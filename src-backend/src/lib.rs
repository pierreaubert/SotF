pub mod audio;
pub use audio::SharedAudioState;

pub mod camilla;
pub use camilla::{
    AudioManager, AudioState, AudioStreamState, CamillaError, CamillaResult, FilterParams,
    SharedAudioStreamState,
};

pub mod optim;
pub mod plot;

// Re-export commonly used types and helpers for easier access in tests and consumers
pub use optim::{
    CancellationState, OptimizationParams, OptimizationResult, ProgressUpdate, validate_params,
};
pub use plot::{CurveData, PlotData, curve_data_to_curve};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_mocks;
