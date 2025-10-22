use autoeq_backend::spinorama_api::SpinAudioClient;
use autoeq_backend::{CurveData, OptimizationParams, OptimizationResult};
use gpui::*;

pub struct SpeakerWorkflow {
    current_step: usize,
    selected_speaker: Option<String>, // "Brand - Model"
    selected_measurement: Option<String>,
    measurement_curve: Option<CurveData>,
    num_filters: usize,
    algorithm: String,
    population: usize,
    maxeval: usize,
    optimization_result: Option<OptimizationResult>,
    api_client: SpinAudioClient,
}

impl SpeakerWorkflow {
    pub fn new() -> Self {
        Self {
            current_step: 0,
            selected_speaker: None,
            selected_measurement: None,
            measurement_curve: None,
            num_filters: 7,
            algorithm: "nlopt:cobyla".to_string(),
            population: 300,
            maxeval: 2000,
            optimization_result: None,
            api_client: SpinAudioClient::new(),
        }
    }

    pub fn render_content() -> impl IntoElement {
        div().child("Speaker Workflow (Stateful version)")
    }

    fn next_step(&mut self, cx: &mut Context<Self>) {
        if self.current_step < 4 {
            self.current_step += 1;
            cx.notify();
        }
    }

    fn prev_step(&mut self, cx: &mut Context<Self>) {
        if self.current_step > 0 {
            self.current_step -= 1;
            cx.notify();
        }
    }

    fn load_demo_speaker(&mut self, cx: &mut Context<Self>) {
        // Load demo speaker data for testing
        self.selected_speaker = Some("KEF LS50 Meta".to_string());
        self.selected_measurement = Some("ASR - Listening Window".to_string());

        // Generate synthetic speaker response curve
        let freqs: Vec<f64> = (0..100)
            .map(|i| 20.0 * (20000.0 / 20.0_f64).powf(i as f64 / 99.0))
            .collect();

        let mags: Vec<f64> = freqs
            .iter()
            .map(|f| {
                // Simulate typical speaker response with room gain and directivity
                let log_f = f.log10();
                let room_gain = if *f < 200.0 {
                    6.0 * (200.0 - f) / 180.0
                } else {
                    0.0
                };
                let presence_peak = if *f > 2000.0 && *f < 4000.0 { 2.0 } else { 0.0 };
                room_gain + presence_peak - 1.0 * (log_f - 3.0).sin()
            })
            .collect();

        self.measurement_curve = Some(CurveData {
            freq: freqs,
            spl: mags,
        });

        cx.notify();
    }

    fn run_optimization(&mut self, _cx: &mut Context<Self>) {
        if self.measurement_curve.is_none() {
            log::warn!("No measurement curve loaded");
            return;
        }

        log::info!("Starting speaker optimization synchronously");

        let curve = self.measurement_curve.clone().unwrap();
        let _params = OptimizationParams {
            num_filters: self.num_filters,
            curve_path: None,
            target_path: None,
            sample_rate: 48000.0,
            max_db: 3.0,
            min_db: 1.0,
            max_q: 3.0,
            min_q: 1.0,
            min_freq: 60.0,
            max_freq: 16000.0,
            speaker: None,
            version: None,
            measurement: None,
            curve_name: "Listening Window".to_string(),
            algo: self.algorithm.clone(),
            population: self.population,
            maxeval: self.maxeval,
            refine: false,
            local_algo: "cobyla".to_string(),
            min_spacing_oct: 0.5,
            spacing_weight: 20.0,
            smooth: true,
            smooth_n: 1,
            loss: "flat".to_string(),
            peq_model: Some("pk".to_string()),
            strategy: Some("currenttobest1bin".to_string()),
            de_f: Some(0.8),
            de_cr: Some(0.9),
            adaptive_weight_f: Some(0.8),
            adaptive_weight_cr: Some(0.7),
            tolerance: Some(1e-3),
            atolerance: Some(1e-4),
            captured_frequencies: Some(curve.freq),
            captured_magnitudes: Some(curve.spl),
            target_frequencies: None,
            target_magnitudes: None,
        };

        // Note: Backend run_optimization requires additional parameters like progress callback and cancellation state
        // For now, we'll create a placeholder result until we implement the full integration
        let result = OptimizationResult {
            success: false,
            error_message: Some("Backend integration in progress - placeholder result".to_string()),
            filter_params: None,
            objective_value: None,
            preference_score_before: None,
            preference_score_after: None,
            filter_response: None,
            spin_details: None,
            filter_plots: None,
            input_curve: None,
            deviation_curve: None,
        };

        self.optimization_result = Some(result);
    }
}

impl Render for SpeakerWorkflow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let steps = vec![
            "Select Speaker",
            "EQ Design",
            "Display CEA2034",
            "Audio Player",
            "Save EQ",
        ];
        let total = steps.len();

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(self.render_step_navigator(&steps))
            .child(self.render_step_content(&steps, cx))
            .child(self.render_navigation_buttons(total, cx))
    }
}

impl SpeakerWorkflow {
    fn render_step_navigator(&self, steps: &[&str]) -> Div {
        div()
            .flex()
            .flex_row()
            .gap_2()
            .items_center()
            .p_4()
            .bg(rgb(0xffffff))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xdddddd))
            .children((0..steps.len()).map(|i| {
                let is_active = i == self.current_step;
                let is_done = i < self.current_step;

                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(32.0))
                    .h(px(32.0))
                    .rounded(px(16.0))
                    .bg(if is_active {
                        rgb(0x4a90e2)
                    } else if is_done {
                        rgb(0x50c878)
                    } else {
                        rgb(0xcccccc)
                    })
                    .text_color(rgb(0xffffff))
                    .font_weight(FontWeight::BOLD)
                    .child((i + 1).to_string())
            }))
    }

    fn render_step_content(&mut self, steps: &[&str], cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_6()
            .bg(rgb(0xffffff))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xdddddd))
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x333333))
                    .child(format!(
                        "Step {}: {}",
                        self.current_step + 1,
                        steps[self.current_step]
                    )),
            )
            .child(match self.current_step {
                0 => self.render_step_1(cx),
                1 => self.render_step_2(cx),
                2 => self.render_step_3(),
                3 => self.render_step_4(),
                4 => self.render_step_5(),
                _ => div().child("Unknown step"),
            })
    }

    fn render_navigation_buttons(&mut self, total: usize, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .flex_row()
            .gap_4()
            .justify_between()
            .child(
                div()
                    .px_6()
                    .py_3()
                    .rounded(px(6.0))
                    .bg(if self.current_step > 0 {
                        rgb(0x4a90e2)
                    } else {
                        rgb(0xcccccc)
                    })
                    .text_color(rgb(0xffffff))
                    .cursor(if self.current_step > 0 {
                        CursorStyle::PointingHand
                    } else {
                        CursorStyle::Arrow
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            if this.current_step > 0 {
                                this.prev_step(cx);
                            }
                        }),
                    )
                    .child("← Previous"),
            )
            .child(
                div()
                    .px_6()
                    .py_3()
                    .rounded(px(6.0))
                    .bg(if self.current_step < total - 1 {
                        rgb(0x4a90e2)
                    } else {
                        rgb(0x50c878)
                    })
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.next_step(cx);
                        }),
                    )
                    .child(if self.current_step < total - 1 {
                        "Next →"
                    } else {
                        "Finish ✓"
                    }),
            )
    }

    fn render_step_1(&mut self, cx: &mut Context<Self>) -> Div {
        let has_speaker = self.selected_speaker.is_some();

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(div().text_color(rgb(0x666666)).child("Select a speaker from spinorama.org database"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().font_weight(FontWeight::SEMIBOLD).child("Speaker:"))
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(if has_speaker { rgb(0x50c878) } else { rgb(0xcccccc) })
                            .cursor_pointer()
                            .hover(|s| s.border_color(rgb(0x4a90e2)))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.load_demo_speaker(cx);
                            }))
                            .child(if let Some(speaker) = &self.selected_speaker {
                                format!("✓ {} (click to change)", speaker)
                            } else {
                                "Click to load demo speaker".to_string()
                            })
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().font_weight(FontWeight::SEMIBOLD).child("Measurement:"))
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(if self.selected_measurement.is_some() { rgb(0x50c878) } else { rgb(0xcccccc) })
                            .text_color(rgb(0x666666))
                            .child(self.selected_measurement.as_ref()
                                .map(|m| m.clone())
                                .unwrap_or_else(|| "Select speaker first".to_string()))
                    )
            )
            .child(
                div()
                    .p_3()
                    .rounded(px(4.0))
                    .bg(rgb(0xfff9e6))
                    .border_1()
                    .border_color(rgb(0xffe4a3))
                    .text_sm()
                    .text_color(rgb(0x856404))
                    .child("💡 Tip: Real spinorama.org integration coming soon. For now, using demo data.")
            )
    }

    fn render_step_2(&mut self, cx: &mut Context<Self>) -> Div {
        let can_optimize = self.measurement_curve.is_some();

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .child("Configure and run speaker EQ optimization"),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(div().text_sm().child("Number of Filters:"))
                            .child(
                                div()
                                    .px_3()
                                    .py_2()
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(rgb(0xcccccc))
                                    .child(format!("{}", self.num_filters)),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(div().text_sm().child("Algorithm:"))
                            .child(
                                div()
                                    .px_3()
                                    .py_2()
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(rgb(0xcccccc))
                                    .child(self.algorithm.clone()),
                            ),
                    ),
            )
            .child(
                div()
                    .px_6()
                    .py_3()
                    .rounded(px(6.0))
                    .bg(if can_optimize {
                        rgb(0x50c878)
                    } else {
                        rgb(0xcccccc)
                    })
                    .text_color(rgb(0xffffff))
                    .cursor(if can_optimize {
                        CursorStyle::PointingHand
                    } else {
                        CursorStyle::Arrow
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            if this.measurement_curve.is_some() {
                                this.run_optimization(cx);
                            }
                        }),
                    )
                    .child("▶ Run Optimization"),
            )
            .child(
                div()
                    .p_4()
                    .rounded(px(4.0))
                    .bg(if self.optimization_result.is_some() {
                        rgb(0xf0f8ff)
                    } else {
                        rgb(0xf9f9f9)
                    })
                    .border_1()
                    .border_color(if self.optimization_result.is_some() {
                        rgb(0xb0d4ff)
                    } else {
                        rgb(0xeeeeee)
                    })
                    .text_sm()
                    .text_color(rgb(0x666666))
                    .child(if let Some(result) = &self.optimization_result {
                        if result.success {
                            format!(
                                "✓ Optimization complete! {} filters generated",
                                result.filter_params.as_ref().map(|f| f.len()).unwrap_or(0)
                            )
                        } else {
                            format!(
                                "✗ Optimization failed: {}",
                                result
                                    .error_message
                                    .as_ref()
                                    .unwrap_or(&"Unknown error".to_string())
                            )
                        }
                    } else if can_optimize {
                        "Ready to optimize speaker response".to_string()
                    } else {
                        "Please select a speaker in Step 1 first".to_string()
                    }),
            )
    }

    fn render_step_3(&mut self) -> Div {
        if let Some(result) = &self.optimization_result {
            if result.success {
                if let Some(filters) = &result.filter_params {
                    return div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .child(
                            div()
                                .text_color(rgb(0x666666))
                                .child("View CEA2034 curves and optimized response"),
                        )
                        .child(
                            div()
                                .w_full()
                                .h(px(350.0))
                                .rounded(px(4.0))
                                .border_1()
                                .border_color(rgb(0xdddddd))
                                .bg(rgb(0xffffff))
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .gap_2()
                                .text_color(rgb(0x666666))
                                .child("[CEA2034 Curves Placeholder]")
                                .child(div().text_sm().child(format!(
                                    "On-Axis, Listening Window, Early Reflections, Sound Power"
                                ))),
                        )
                        .child(
                            div()
                                .p_4()
                                .rounded(px(4.0))
                                .bg(rgb(0xf0f8ff))
                                .border_1()
                                .border_color(rgb(0xb0d4ff))
                                .text_sm()
                                .child(format!(
                                    "ℹ Speaker optimized with {} parametric filters",
                                    filters.len()
                                )),
                        );
                }
            }
        }

        div()
            .flex()
            .items_center()
            .justify_center()
            .h(px(200.0))
            .text_color(rgb(0x999999))
            .child("Run optimization in Step 2 to see CEA2034 curves")
    }

    fn render_step_4(&mut self) -> Div {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .child("Preview optimized speaker with audio (Coming soon)"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(150.0))
                    .rounded(px(4.0))
                    .border_1()
                    .border_color(rgb(0xdddddd))
                    .bg(rgb(0xfafafa))
                    .text_color(rgb(0x999999))
                    .child("[Audio Player Component]"),
            )
    }

    fn render_step_5(&mut self) -> Div {
        let has_filters = self
            .optimization_result
            .as_ref()
            .and_then(|r| r.filter_params.as_ref())
            .is_some();

        if !has_filters {
            return div()
                .flex()
                .items_center()
                .justify_center()
                .h(px(200.0))
                .text_color(rgb(0x999999))
                .child("Run optimization first to export filters");
        }

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .child("Save your speaker EQ settings"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Export Format:"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(rgb(0xcccccc))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0xf5f5f5)))
                                    .child("◉ CamillaDSP"),
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(rgb(0xcccccc))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0xf5f5f5)))
                                    .child("○ Parametric EQ"),
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(rgb(0xcccccc))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0xf5f5f5)))
                                    .child("○ REW (Room EQ Wizard)"),
                            ),
                    ),
            )
            .child(
                div()
                    .px_6()
                    .py_3()
                    .rounded(px(6.0))
                    .bg(rgb(0x50c878))
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgb(0x40b868)))
                    .child("💾 Save Speaker EQ Configuration"),
            )
    }
}
