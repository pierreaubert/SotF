use autoeq_backend::audio::{AudioDevice, AudioConfig};
use autoeq_backend::{CancellationState, SharedAudioState};
use tauri::State;

#[tauri::command]
fn exit_app(window: tauri::Window) {
    window.close().unwrap();
}

// ============================================================================
// Audio Device Management Commands (Tauri wrappers for backend functions)
// ============================================================================

#[tauri::command]
async fn get_audio_devices() -> Result<std::collections::HashMap<String, Vec<AudioDevice>>, String> {
    autoeq_backend::audio::get_audio_devices()
}

#[tauri::command]
async fn set_audio_device(
    device_name: String,
    is_input: bool,
    config: AudioConfig,
    audio_state: State<'_, SharedAudioState>,
) -> Result<String, String> {
    autoeq_backend::audio::set_audio_device(
        device_name,
        is_input,
        config,
        &*audio_state,
    )
}

#[tauri::command]
async fn get_audio_config(
    audio_state: State<'_, SharedAudioState>,
) -> Result<autoeq_backend::audio::AudioState, String> {
    autoeq_backend::audio::get_audio_config(&*audio_state)
}

#[tauri::command]
async fn get_device_properties(
    device_name: String,
    is_input: bool,
) -> Result<serde_json::Value, String> {
    autoeq_backend::audio::get_device_properties(device_name, is_input)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(CancellationState::new())
        .manage(SharedAudioState::default())
        .invoke_handler(tauri::generate_handler![
            exit_app,
            get_audio_devices,
            set_audio_device,
            get_audio_config,
            get_device_properties
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
