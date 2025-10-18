use autoeq_backend::audio;
use autoeq_backend::{CancellationState, SharedAudioState};

#[tauri::command]
fn exit_app(window: tauri::Window) {
    window.close().unwrap();
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
            audio::get_audio_devices,
            audio::set_audio_device,
            audio::get_audio_config,
            audio::get_device_properties
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
