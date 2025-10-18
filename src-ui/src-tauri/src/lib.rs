use autoeq::{
    Curve, LossType, cli::Args as AutoEQArgs, plot_filters, plot_spin, plot_spin_details,
    plot_spin_tonal,
};
use ndarray::Array1;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, State};

// Import from autoeq_backend
use autoeq_backend::{
    CancellationState, OptimizationParams, OptimizationResult, PlotFiltersParams, PlotSpinParams,
    SharedAudioState, audio, curve_data_to_curve, plot_to_json, run_optimization_internal,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_speakers() -> Result<Vec<String>, String> {
    match reqwest::get("https://api.spinorama.org/v1/speakers").await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(data) => {
                if let Some(speakers) = data.as_array() {
                    let speaker_names: Vec<String> = speakers
                        .iter()
                        .filter_map(|s| s.as_str())
                        .map(|s| s.to_string())
                        .collect();
                    Ok(speaker_names)
                } else {
                    Err("Invalid response format".to_string())
                }
            }
            Err(e) => Err(format!("Failed to parse response: {}", e)),
        },
        Err(e) => Err(format!("Failed to fetch speakers: {}", e)),
    }
}

#[tauri::command]
async fn get_speaker_versions(speaker: String) -> Result<Vec<String>, String> {
    let url = format!(
        "https://api.spinorama.org/v1/speaker/{}/versions",
        urlencoding::encode(&speaker)
    );
    match reqwest::get(&url).await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(data) => {
                if let Some(versions) = data.as_array() {
                    let version_names: Vec<String> = versions
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(|v| v.to_string())
                        .collect();
                    Ok(version_names)
                } else {
                    Err("Invalid response format".to_string())
                }
            }
            Err(e) => Err(format!("Failed to parse response: {}", e)),
        },
        Err(e) => Err(format!("Failed to fetch versions: {}", e)),
    }
}

#[tauri::command]
async fn get_speaker_measurements(speaker: String, version: String) -> Result<Vec<String>, String> {
    let url = format!(
        "https://api.spinorama.org/v1/speaker/{}/version/{}/measurements",
        urlencoding::encode(&speaker),
        urlencoding::encode(&version)
    );
    match reqwest::get(&url).await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(data) => {
                if let Some(measurements) = data.as_array() {
                    let measurement_names: Vec<String> = measurements
                        .iter()
                        .filter_map(|m| m.as_str())
                        .map(|m| m.to_string())
                        .collect();
                    Ok(measurement_names)
                } else {
                    Err("Invalid response format".to_string())
                }
            }
            Err(e) => Err(format!("Failed to parse response: {}", e)),
        },
        Err(e) => Err(format!("Failed to fetch measurements: {}", e)),
    }
}

#[tauri::command]
async fn run_optimization(
    params: OptimizationParams,
    app_handle: AppHandle,
    cancellation_state: State<'_, CancellationState>,
) -> Result<OptimizationResult, String> {
    println!(
        "[RUST DEBUG] run_optimization called with algo: {}",
        params.algo
    );
    println!(
        "[RUST DEBUG] Parameters: num_filters={}, population={}, maxeval={}",
        params.num_filters, params.population, params.maxeval
    );

    // Reset cancellation state at the start of optimization
    cancellation_state.reset();

    let result =
        run_optimization_internal(params, app_handle, Arc::new((*cancellation_state).clone()))
            .await;
    match result {
        Ok(res) => {
            println!("[RUST DEBUG] Optimization completed successfully");
            Ok(res)
        }
        Err(e) => {
            println!("[RUST DEBUG] Optimization failed with error: {}", e);
            Ok(OptimizationResult {
                success: false,
                error_message: Some(e.to_string()),
                filter_params: None,
                objective_value: None,
                preference_score_before: None,
                preference_score_after: None,
                filter_response: None,
                spin_details: None,
                filter_plots: None,
                input_curve: None,
                deviation_curve: None,
            })
        }
    }
}

#[tauri::command]
async fn generate_plot_filters(params: PlotFiltersParams) -> Result<serde_json::Value, String> {
    // Convert CurveData to autoeq::Curve
    let input_curve = curve_data_to_curve(&params.input_curve);
    let target_curve = curve_data_to_curve(&params.target_curve);
    let deviation_curve = curve_data_to_curve(&params.deviation_curve);

    // Create a minimal Args struct for the plot function
    let args = AutoEQArgs {
        num_filters: params.num_filters,
        curve: None,
        target: None,
        sample_rate: params.sample_rate,
        max_db: 3.0,
        min_db: 1.0,
        max_q: 3.0,
        min_q: 1.0,
        min_freq: 60.0,
        max_freq: 16000.0,
        output: None,
        speaker: None,
        version: None,
        measurement: None,
        curve_name: "Listening Window".to_string(),
        algo: "nlopt:cobyla".to_string(),
        population: 300,
        maxeval: 2000,
        refine: false,
        local_algo: "cobyla".to_string(),
        min_spacing_oct: 0.5,
        spacing_weight: 20.0,
        smooth: true,
        smooth_n: 2,
        loss: LossType::SpeakerFlat,
        peq_model: match params.peq_model.as_deref() {
            Some("hp-pk") => autoeq::cli::PeqModel::HpPk,
            Some("hp-pk-lp") => autoeq::cli::PeqModel::HpPkLp,
            Some("free-pk-free") => autoeq::cli::PeqModel::FreePkFree,
            Some("free") => autoeq::cli::PeqModel::Free,
            Some("pk") | _ => autoeq::cli::PeqModel::Pk,
        },
        peq_model_list: false,
        algo_list: false,
        tolerance: 1e-3,
        atolerance: 1e-4,
        recombination: 0.9,
        strategy: "currenttobest1bin".to_string(),
        strategy_list: false,
        adaptive_weight_f: 0.9,
        adaptive_weight_cr: 0.9,
        no_parallel: false,
        parallel_threads: 0,
        qa: false, // Quality assurance mode disabled for UI
    };

    // Generate the plot
    let plot = plot_filters(
        &args,
        &input_curve,
        &target_curve,
        &deviation_curve,
        &params.optimized_params,
    );

    // Convert to JSON
    plot_to_json(plot)
}

#[tauri::command]
async fn generate_plot_spin(params: PlotSpinParams) -> Result<serde_json::Value, String> {
    // Convert CurveData HashMap to autoeq::Curve HashMap if provided
    let cea2034_curves = params.cea2034_curves.as_ref().map(|curves| {
        curves
            .iter()
            .map(|(name, curve_data)| (name.clone(), curve_data_to_curve(curve_data)))
            .collect::<HashMap<String, Curve>>()
    });

    // Convert eq_response to Array1 if provided
    let eq_response = params
        .eq_response
        .as_ref()
        .map(|response| Array1::from_vec(response.clone()));

    // Generate the plot
    let plot = plot_spin(cea2034_curves.as_ref(), eq_response.as_ref());

    // Convert to JSON
    plot_to_json(plot)
}

#[tauri::command]
async fn generate_plot_spin_details(params: PlotSpinParams) -> Result<serde_json::Value, String> {
    // Convert CurveData HashMap to autoeq::Curve HashMap if provided
    let cea2034_curves = params.cea2034_curves.as_ref().map(|curves| {
        curves
            .iter()
            .map(|(name, curve_data)| (name.clone(), curve_data_to_curve(curve_data)))
            .collect::<HashMap<String, Curve>>()
    });

    // Convert eq_response to Array1 if provided
    let eq_response = params
        .eq_response
        .as_ref()
        .map(|response| Array1::from_vec(response.clone()));

    // Generate the plot
    let plot = plot_spin_details(cea2034_curves.as_ref(), eq_response.as_ref());

    // Convert to JSON
    plot_to_json(plot)
}

#[tauri::command]
async fn generate_plot_spin_tonal(params: PlotSpinParams) -> Result<serde_json::Value, String> {
    // Convert CurveData HashMap to autoeq::Curve HashMap if provided
    let cea2034_curves = params.cea2034_curves.as_ref().map(|curves| {
        curves
            .iter()
            .map(|(name, curve_data)| (name.clone(), curve_data_to_curve(curve_data)))
            .collect::<HashMap<String, Curve>>()
    });

    // Convert eq_response to Array1 if provided
    let eq_response = params
        .eq_response
        .as_ref()
        .map(|response| Array1::from_vec(response.clone()));

    // Generate the plot
    let plot = plot_spin_tonal(cea2034_curves.as_ref(), eq_response.as_ref());

    // Convert to JSON
    plot_to_json(plot)
}

#[tauri::command]
fn exit_app(window: tauri::Window) {
    window.close().unwrap();
}

#[tauri::command]
fn cancel_optimization(cancellation_state: State<CancellationState>) -> Result<(), String> {
    println!("[RUST DEBUG] Cancellation requested");
    cancellation_state.cancel();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(CancellationState::new())
        .manage(SharedAudioState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            run_optimization,
            cancel_optimization,
            get_speakers,
            get_speaker_versions,
            get_speaker_measurements,
            generate_plot_filters,
            generate_plot_spin,
            generate_plot_spin_details,
            generate_plot_spin_tonal,
            exit_app,
            audio::get_audio_devices,
            audio::set_audio_device,
            audio::get_audio_config,
            audio::get_device_properties
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
