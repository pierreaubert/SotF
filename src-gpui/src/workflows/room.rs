use autoeq_backend::{CurveData, OptimizationParams, OptimizationResult};
use gpui::*;

pub struct RoomWorkflow {
    current_step: usize,
    room_measurement: Option<CurveData>,
    corrected_response: Option<CurveData>,
    num_filters: usize,
    algorithm: String,
    population: usize,
    maxeval: usize,
    optimization_result: Option<OptimizationResult>,
    is_recording: bool,
}

impl RoomWorkflow {
    pub fn new() -> Self {
        Self {
            current_step: 0,
            room_measurement: None,
            corrected_response: None,
            num_filters: 10, // Rooms typically need more filters for modes
            algorithm: "nlopt:cobyla".to_string(),
            population: 300,
            maxeval: 2000,
            optimization_result: None,
            is_recording: false,
        }
    }

    pub fn render_content() -> impl IntoElement {
        div().child("Room Workflow (Stateful version)")
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

    fn simulate_room_measurement(&mut self, cx: &mut Context<Self>) {
        // Simulate room measurement with typical acoustic issues
        log::info!("Simulating room measurement");

        let freqs: Vec<f64> = (0..100)
            .map(|i| 20.0 * (20000.0 / 20.0_f64).powf(i as f64 / 99.0))
            .collect();

        let mags: Vec<f64> = freqs
            .iter()
            .map(|f| {
                // Simulate typical room response issues
                let log_f = f.log10();

                // Room modes (peaks and nulls at low frequencies)
                let room_modes = if *f < 300.0 {
                    let mode_1 = 8.0 * ((*f - 50.0) / 30.0).sin();
                    let mode_2 = 5.0 * ((*f - 80.0) / 25.0).sin();
                    mode_1 + mode_2
                } else {
                    0.0
                };

                // SBIR dip (Speaker Boundary Interference Response)
                let sbir_dip = if *f > 100.0 && *f < 200.0 {
                    -6.0 * ((*f - 150.0) / 50.0).powi(2).exp()
                } else {
                    0.0
                };

                // General room gain at low frequencies
                let room_gain = if *f < 100.0 {
                    4.0 * (100.0 - f) / 80.0
                } else {
                    0.0
                };

                // High frequency absorption
                let hf_absorption = if *f > 8000.0 {
                    -2.0 * ((*f - 8000.0) / 12000.0)
                } else {
                    0.0
                };

                room_modes + sbir_dip + room_gain + hf_absorption + 0.5 * (log_f - 2.8).sin()
            })
            .collect();

        self.room_measurement = Some(CurveData {
            freq: freqs,
            spl: mags,
        });

        cx.notify();
    }

    fn run_optimization(&mut self, _cx: &mut Context<Self>) {
        if self.room_measurement.is_none() {
            log::warn!("No room measurement available");
            return;
        }

        log::info!("Starting room correction optimization");

        let curve = self.room_measurement.clone().unwrap();
        let _params = OptimizationParams {
            num_filters: self.num_filters,
            curve_path: None,
            target_path: None,
            sample_rate: 48000.0,
            max_db: 6.0,
            min_db: 1.0,
            max_q: 10.0,
            min_q: 1.0,
            min_freq: 20.0,
            max_freq: 500.0, // Focus on low frequencies for room correction
            speaker: None,
            version: None,
            measurement: None,
            curve_name: "Listening Window".to_string(),
            algo: self.algorithm.clone(),
            population: self.population,
            maxeval: self.maxeval,
            refine: false,
            local_algo: "cobyla".to_string(),
            min_spacing_oct: 0.25,
            spacing_weight: 10.0,
            smooth: true,
            smooth_n: 3,
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

impl Render for RoomWorkflow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let steps = vec![
            "Audio Capture",
            "EQ Design",
            "Display Curves",
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

impl RoomWorkflow {
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
        let has_measurement = self.room_measurement.is_some();

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .child("Capture room measurement with sweep signal"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .p_4()
                            .rounded(px(4.0))
                            .bg(rgb(0xfff9e6))
                            .border_1()
                            .border_color(rgb(0xffe4a3))
                            .text_sm()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x856404))
                                            .child("🎤 Room Measurement Steps:"),
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x856404))
                                            .child("1. Position microphone at listening position"),
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x856404))
                                            .child("2. Play logarithmic sweep through speakers"),
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x856404))
                                            .child("3. Record and analyze room response"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .px_6()
                            .py_3()
                            .rounded(px(6.0))
                            .bg(if has_measurement {
                                rgb(0x50c878)
                            } else {
                                rgb(0xff6b6b)
                            })
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| {
                                s.bg(if has_measurement {
                                    rgb(0x40b868)
                                } else {
                                    rgb(0xff5252)
                                })
                            })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.simulate_room_measurement(cx);
                                }),
                            )
                            .child(if has_measurement {
                                "✓ Measurement Complete (click to remeasure)"
                            } else {
                                "🎧 Start Room Measurement (Demo)"
                            }),
                    ),
            )
            .child(
                div()
                    .p_3()
                    .rounded(px(4.0))
                    .bg(rgb(0xe8f4ff))
                    .border_1()
                    .border_color(rgb(0xb3d9ff))
                    .text_sm()
                    .text_color(rgb(0x004085))
                    .child(if has_measurement {
                        "✓ Room measurement captured successfully!"
                    } else {
                        "💡 Note: Real audio capture coming soon. Using simulated data for demo."
                    }),
            )
    }

    fn render_step_2(&mut self, cx: &mut Context<Self>) -> Div {
        let can_optimize = self.room_measurement.is_some();

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(div().text_color(rgb(0x666666)).child("Design room correction EQ"))
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
                                    .child(format!("{}", self.num_filters))
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(div().text_sm().child("Target Range:"))
                            .child(
                                div()
                                    .px_3()
                                    .py_2()
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(rgb(0xcccccc))
                                    .child("20-500 Hz")
                            )
                    )
            )
            .child(
                div()
                    .px_6()
                    .py_3()
                    .rounded(px(6.0))
                    .bg(if can_optimize { rgb(0x50c878) } else { rgb(0xcccccc) })
                    .text_color(rgb(0xffffff))
                    .cursor(if can_optimize { CursorStyle::PointingHand } else { CursorStyle::Arrow })
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                        if this.room_measurement.is_some() {
                            this.run_optimization(cx);
                        }
                    }))
                    .child("▶ Run Room Correction")
            )
            .child(
                div()
                    .p_4()
                    .rounded(px(4.0))
                    .bg(if self.optimization_result.is_some() { rgb(0xf0f8ff) } else { rgb(0xf9f9f9) })
                    .border_1()
                    .border_color(if self.optimization_result.is_some() { rgb(0xb0d4ff) } else { rgb(0xeeeeee) })
                    .text_sm()
                    .text_color(rgb(0x666666))
                    .child(if let Some(result) = &self.optimization_result {
                        if result.success {
                            format!("✓ Room correction optimized! {} filters targeting room modes", 
                                result.filter_params.as_ref().map(|f| f.len()).unwrap_or(0))
                        } else {
                            format!("✗ Optimization failed: {}", result.error_message.as_ref().unwrap_or(&"Unknown error".to_string()))
                        }
                    } else if can_optimize {
                        "Ready to optimize room correction".to_string()
                    } else {
                        "Please capture room measurement in Step 1 first".to_string()
                    })
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
                    .child("🎯 Focus: Correcting room modes and SBIR (Speaker Boundary Interference Response)")
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
                                .child("Compare measured vs corrected room response"),
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
                                .child("[Room Response Comparison Plot]")
                                .child(
                                    div().text_sm().child("Measured (red) vs Corrected (green)"),
                                ),
                        )
                        .child(
                            div()
                                .p_4()
                                .rounded(px(4.0))
                                .bg(rgb(0xf0f8ff))
                                .border_1()
                                .border_color(rgb(0xb0d4ff))
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child(format!("ℹ Room Correction Summary")),
                                )
                                .child(div().text_sm().child(format!(
                                    "• {} parametric filters applied",
                                    filters.len()
                                )))
                                .child(div().text_sm().child("• Targeting room modes below 500 Hz"))
                                .child(div().text_sm().child("• SBIR nulls minimized")),
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
            .child("Run room correction in Step 2 to see comparison")
    }

    fn render_step_4(&mut self) -> Div {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .child("Preview room correction with audio (Coming soon)"),
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
                    .child("[Audio Player with Room Correction Toggle]"),
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
                .child("Run room correction first to export filters");
        }

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .child("Save your room correction settings"),
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
                    .child("💾 Save Room Correction Configuration"),
            )
    }
}
