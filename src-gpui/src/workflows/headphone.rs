use crate::components::audio_interface::AudioInterfaceComponent;
use crate::components::eq_design::OptimizationStatus;
use crate::components::eq_design::EQDesignComponent;
use crate::components::filter_display::FilterDisplayComponent;
use crate::components::frequency_plot::FrequencyPlotComponent;
use autoeq_backend::export::{export_filters, ExportFormat, FilterParam as ExportFilterParam};
use autoeq_backend::{CurveData, OptimizationResult};
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::path::PathBuf;

pub struct HeadphoneWorkflow {
    current_step: usize,
    input_curve: Option<CurveData>,
    target_curve: Option<CurveData>,
    input_file_path: Option<PathBuf>,
    optimization_result: Option<OptimizationResult>,
    is_optimizing: bool,
    plot_component: Entity<FrequencyPlotComponent>,
    filter_display: Entity<FilterDisplayComponent>,
    audio_interface: Entity<AudioInterfaceComponent>,
    eq_design: Entity<EQDesignComponent>,
    // Target curve selection
    available_targets: Vec<String>,
    selected_target_index: usize, // 0 = Flat, 1+ = target curves
    show_target_dropdown: bool,
}

impl HeadphoneWorkflow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let plot_component = cx.new(|_cx| FrequencyPlotComponent::new(600.0, 400.0));
        let filter_display = cx.new(|_cx| FilterDisplayComponent::new(Vec::new()));
        let audio_interface = cx.new(|cx| AudioInterfaceComponent::new(cx));
        let eq_design = cx.new(|_cx| {
            let component = EQDesignComponent::new();
            // Set headphone-specific defaults
            component
        });

        // Discover available target curves from public/headphone-targets
        let mut available_targets = vec!["Flat (0 dB)".to_string()];
        
        // Try to read target curve directory (relative to project root)
        let target_dir = PathBuf::from("../public/headphone-targets");
        if let Ok(entries) = std::fs::read_dir(&target_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".csv") {
                        // Convert file name to friendly display name
                        // e.g., "harman-in-ear-2019.csv" -> "Harman In-Ear 2019"
                        let display_name = file_name
                            .trim_end_matches(".csv")
                            .split('-')
                            .map(|part| {
                                let mut chars = part.chars();
                                match chars.next() {
                                    None => String::new(),
                                    Some(first) => {
                                        first.to_uppercase().collect::<String>() + chars.as_str()
                                    }
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(" ");
                        available_targets.push(display_name);
                    }
                }
            }
        } else {
            log::warn!("Could not read headphone targets directory: {:?}", target_dir);
        }

        log::info!("Found {} target curves", available_targets.len());

        Self {
            current_step: 0,
            input_curve: None,
            target_curve: None,
            input_file_path: None,
            optimization_result: None,
            is_optimizing: false,
            plot_component,
            filter_display,
            audio_interface,
            eq_design,
            available_targets,
            selected_target_index: 0, // Default to Flat
            show_target_dropdown: false,
        }
    }

    pub fn render_content() -> impl IntoElement {
        div().child("Headphone Workflow (Stateful version coming)")
    }

    fn go_to_step(&mut self, step: usize, cx: &mut Context<Self>) {
        if step < 5 {
            self.current_step = step;
            cx.notify();
        }
    }

    fn next_step(&mut self, cx: &mut Context<Self>) {
        if self.current_step < 4 {
            self.current_step += 1;

            // Update results when moving to step 3 (Display Curve)
            if self.current_step == 2 {
                self.update_from_eq_design_results(cx);
            }

            cx.notify();
        }
    }

    fn prev_step(&mut self, cx: &mut Context<Self>) {
        if self.current_step > 0 {
            self.current_step -= 1;
            cx.notify();
        }
    }

    fn update_from_eq_design_results(&mut self, cx: &mut Context<Self>) {
        // Extract results from EQDesignComponent first (without nested borrows)
        let optimization_data = self.eq_design.update(cx, |eq, _cx| {
            if let OptimizationStatus::Success(result) = eq.optimization_status() {
                Some(result.clone())
            } else {
                None
            }
        });

        // If we have a result, process it outside the closure
        if let Some(result) = optimization_data {
            // Store the full result
            self.optimization_result = Some(result.clone());

            // Convert filter parameters for display
            if let Some(raw_params) = &result.filter_params {
                let filters = self.convert_raw_params_to_filters(
                    raw_params,
                    raw_params.len() / 3,  // num_filters
                    "pk",  // Default to pk model
                );

                // Update filter display
                self.filter_display.update(cx, |display, cx| {
                    display.set_filters(filters, cx);
                });
            }

            // Update plot component with optimization results
            if let Some(filter_response) = &result.filter_response {
                self.plot_component.update(cx, |plot, cx| {
                    plot.set_filter_response(filter_response.clone(), cx);
                });
            }

            // Create optimized curve from input + filter response
            if let (Some(input), Some(filter_resp)) = (&self.input_curve, &result.filter_response) {
                // Find EQ Response curve in filter_response
                if let Some(eq_response) = filter_resp.curves.get("EQ Response") {
                    // Create optimized curve by adding EQ to input
                    let optimized = CurveData {
                        freq: filter_resp.frequencies.clone(),
                        spl: input.spl.iter().zip(eq_response.iter())
                            .map(|(inp, eq)| inp + eq)
                            .collect(),
                    };
                    self.plot_component.update(cx, |plot, cx| {
                        plot.set_optimized_curve(optimized, cx);
                    });
                }
            }

            cx.notify();
        }
    }

    fn sync_curves_to_eq_design(&mut self, cx: &mut Context<Self>) {
        // Sync input curve
        if let Some(curve) = &self.input_curve {
            self.eq_design.update(cx, |eq, cx| {
                eq.set_input_curve(curve.clone(), cx);
            });
        }

        // Sync target curve
        self.eq_design.update(cx, |eq, cx| {
            eq.set_target_curve(self.target_curve.clone(), cx);
        });
    }

    fn load_selected_target(&mut self, cx: &mut Context<Self>) {
        if self.selected_target_index == 0 {
            // Flat (0 dB) target - clear target curve
            self.target_curve = None;
            log::info!("Selected flat target (0 dB)");

            // Update all components in sequence without nested borrows
            self.plot_component.update(cx, |plot, cx| {
                plot.clear_target_curve(cx);
            });

            // Sync to EQ design after plot update completes
            if let Some(curve) = &self.input_curve {
                let curve_clone = curve.clone();
                self.eq_design.update(cx, |eq, cx| {
                    eq.set_input_curve(curve_clone, cx);
                });
            }
            self.eq_design.update(cx, |eq, cx| {
                eq.set_target_curve(None, cx);
            });

            cx.notify();
            return;
        }

        // Get the display name and convert back to file name
        let display_name = &self.available_targets[self.selected_target_index];

        // Convert display name back to file name
        // e.g., "Harman In-Ear 2019" -> "harman-in-ear-2019.csv"
        let file_name = display_name
            .to_lowercase()
            .replace(' ', "-")
            + ".csv";
        
        let file_path = PathBuf::from("../public/headphone-targets").join(&file_name);
        log::info!("Attempting to load target from: {:?}", file_path);

        match self.parse_csv_curve(&file_path) {
            Ok(curve) => {
                log::info!("Loaded target curve from: {:?}", file_path);

                // Store the curve first
                self.target_curve = Some(curve.clone());

                // Update plot component with target curve
                self.plot_component.update(cx, |plot, cx| {
                    plot.set_target_curve(curve.clone(), cx);
                });

                // Sync to EQ design after plot update completes
                self.eq_design.update(cx, |eq, cx| {
                    eq.set_target_curve(Some(curve), cx);
                });

                cx.notify();
            }
            Err(e) => {
                log::error!("Failed to load target curve {}: {}", file_name, e);
                // Revert to flat if loading fails
                self.selected_target_index = 0;
                self.target_curve = None;
                cx.notify();
            }
        }
    }

    fn load_demo_curve(&mut self, cx: &mut Context<Self>) {
        // Load a simple demo curve for testing
        let freqs: Vec<f64> = (0..100)
            .map(|i| 20.0 * (20000.0 / 20.0_f64).powf(i as f64 / 99.0))
            .collect();

        let mags: Vec<f64> = freqs
            .iter()
            .map(|f| {
                // Simulate typical headphone response with some peaks and dips
                let log_f = f.log10();
                2.0 * (log_f - 2.5).sin() + 1.0 * (log_f * 3.0).cos()
            })
            .collect();

        let curve = CurveData {
            freq: freqs,
            spl: mags,
        };

        // Store curve first
        self.input_curve = Some(curve.clone());
        self.input_file_path = None;

        // Update plot component
        self.plot_component.update(cx, |plot, cx| {
            plot.set_input_curve(curve.clone(), cx);
        });

        // Sync to EQ design after plot update completes
        self.eq_design.update(cx, |eq, cx| {
            eq.set_input_curve(curve.clone(), cx);
        });

        // Sync target curve too
        let target = self.target_curve.clone();
        self.eq_design.update(cx, |eq, cx| {
            eq.set_target_curve(target, cx);
        });

        cx.notify();
    }

    fn load_file_curve(&mut self, cx: &mut Context<Self>) {
        // Open file picker dialog synchronously
        let file_path = rfd::FileDialog::new()
            .add_filter("CSV", &["csv"])
            .set_title("Select Measurement CSV File")
            .pick_file();

        if let Some(path) = file_path {
            match self.parse_csv_curve(&path) {
                Ok(curve) => {
                    log::info!("Loaded curve from: {:?}", path);

                    // Store curve and path first
                    self.input_curve = Some(curve.clone());
                    self.input_file_path = Some(path.clone());

                    // Update plot component
                    self.plot_component.update(cx, |plot, cx| {
                        plot.set_input_curve(curve.clone(), cx);
                    });

                    // Sync to EQ design after plot update completes
                    self.eq_design.update(cx, |eq, cx| {
                        eq.set_input_curve(curve.clone(), cx);
                    });

                    // Sync target curve too
                    let target = self.target_curve.clone();
                    self.eq_design.update(cx, |eq, cx| {
                        eq.set_target_curve(target, cx);
                    });

                    cx.notify();
                }
                Err(e) => {
                    log::error!("Failed to parse CSV: {}", e);
                    // TODO: Show error in UI
                }
            }
        }
    }

    fn parse_csv_curve(&self, path: &PathBuf) -> Result<CurveData, String> {
        // Use autoeq library's load_frequency_response function
        // It handles various CSV formats including comma/whitespace separation
        // and different column layouts (2-column and 4-column formats)
        let (freq_array, spl_array) = autoeq::load_frequency_response(path)
            .map_err(|e| format!("Failed to load frequency response: {}", e))?;

        log::info!("Loaded {} data points from CSV", freq_array.len());

        Ok(CurveData {
            freq: freq_array.to_vec(),
            spl: spl_array.to_vec(),
        })
    }

    fn convert_raw_params_to_filters(
        &self,
        raw_params: &[f64],
        num_filters: usize,
        peq_model: &str,
    ) -> Vec<ExportFilterParam> {
        let mut filters = Vec::new();

        for i in 0..num_filters {
            let base_idx = i * 3;
            if base_idx + 2 >= raw_params.len() {
                break;
            }

            // Extract parameters (3 values per filter in log-space)
            let log_freq = raw_params[base_idx];
            let q = raw_params[base_idx + 1];
            let gain = raw_params[base_idx + 2];

            // Convert log frequency to linear: 10^log_freq
            let frequency = 10_f64.powf(log_freq);

            // Determine filter type based on PEQ model and position
            let filter_type = match peq_model {
                "hp-pk" if i == 0 => "HP",
                "hp-pk-lp" if i == 0 => "HP",
                "hp-pk-lp" if i == num_filters - 1 => "LP",
                _ => "PK", // Default to peak filter
            };

            filters.push(ExportFilterParam {
                filter_type: filter_type.to_string(),
                frequency,
                gain,
                q,
            });
        }

        // Sort filters by frequency for better readability
        filters.sort_by(|a, b| a.frequency.partial_cmp(&b.frequency).unwrap());

        filters
    }

    fn export_filters(&self, format: ExportFormat) {
        // Get filter parameters from optimization result
        let raw_params = match &self.optimization_result {
            Some(result) => match &result.filter_params {
                Some(f) => f.clone(),
                None => {
                    log::warn!("No filter parameters to export");
                    return;
                }
            },
            None => {
                log::warn!("No optimization result to export");
                return;
            }
        };

        // Convert raw parameters to FilterParam structs
        let num_filters = raw_params.len() / 3;
        let export_filter_params = self.convert_raw_params_to_filters(&raw_params, num_filters, "pk");

        // Generate export content
        let content = match export_filters(&export_filter_params, format, 48000) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to export filters: {}", e);
                return;
            }
        };

        // Determine file extension and filter name
        let (extension, filter_name) = match format {
            ExportFormat::CamillaDSP => ("yml", "CamillaDSP YAML"),
            ExportFormat::ParametricEQ => ("txt", "Parametric EQ"),
            ExportFormat::REW => ("txt", "REW"),
        };

        // Open save file dialog
        let file_path = rfd::FileDialog::new()
            .add_filter(filter_name, &[extension])
            .set_file_name(format!("autoeq_filters.{}", extension))
            .set_title("Save Filter Configuration")
            .save_file();

        if let Some(path) = file_path {
            match std::fs::write(&path, content) {
                Ok(_) => log::info!("Exported filters to: {:?}", path),
                Err(e) => log::error!("Failed to write file: {}", e),
            }
        }
    }

}

impl Render for HeadphoneWorkflow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let steps = vec![
            "Select Curve & Target",
            "EQ Design",
            "Display Curve",
            "Audio Player",
            "Save EQ",
        ];
        let total = steps.len();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .child(self.render_step_navigator(&steps))
            .child(
                // Scrollable content area (flex_1 makes it take remaining space)
                div()
                    .flex_1()
                    .p_6()
                    .child(self.render_step_content(&steps, cx)),
            )
            .child(
                // Fixed navigation buttons at bottom
                div()
                    .p_4()
                    .border_t_1()
                    .border_color(rgb(0xdddddd))
                    .bg(rgb(0xffffff))
                    .child(self.render_navigation_buttons(total, cx)),
            )
    }
}

impl HeadphoneWorkflow {
    fn render_target_dropdown(&mut self, cx: &mut Context<Self>) -> Div {
        let selected_name = self.available_targets
            .get(self.selected_target_index)
            .cloned()
            .unwrap_or_else(|| "Flat (0 dB)".to_string());

        // Collect dropdown items first to avoid nested borrow issues
        let dropdown_items: Vec<_> = if self.show_target_dropdown {
            self.available_targets
                .iter()
                .enumerate()
                .map(|(idx, name)| {
                    let is_selected = idx == self.selected_target_index;
                    let idx_copy = idx;
                    let name_clone = name.clone();

                    div()
                        .px_4()
                        .py_2()
                        .bg(if is_selected {
                            rgb(0xe3f2fd)
                        } else {
                            rgb(0xffffff)
                        })
                        .cursor_pointer()
                        .hover(|s| s.bg(if is_selected {
                            rgb(0xbbdefb)
                        } else {
                            rgb(0xf5f5f5)
                        }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.selected_target_index = idx_copy;
                                this.show_target_dropdown = false;
                                this.load_selected_target(cx);
                                cx.notify();
                            }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .child(
                                    div().w(px(20.0)).text_center().child(if is_selected {
                                        "●"
                                    } else {
                                        "○"
                                    }),
                                )
                                .child(
                                    div()
                                        .text_color(rgb(0x333333))
                                        .child(name_clone),
                                ),
                        )
                })
                .collect()
        } else {
            Vec::new()
        };

        div()
            .relative()
            .w_full()
            .child(
                // Main dropdown button
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_2()
                    .rounded(px(4.0))
                    .border_1()
                    .border_color(if self.show_target_dropdown {
                        rgb(0x4a90e2)
                    } else {
                        rgb(0xcccccc)
                    })
                    .bg(rgb(0xffffff))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgb(0xf5f5f5)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.show_target_dropdown = !this.show_target_dropdown;
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .text_color(rgb(0x333333))
                            .child(selected_name.clone()),
                    )
                    .child(
                        div()
                            .text_color(rgb(0x666666))
                            .child(if self.show_target_dropdown { "▲" } else { "▼" }),
                    ),
            )
            .when(self.show_target_dropdown, |parent| {
                parent.child(
                    div()
                        .absolute()
                        .top(px(44.0))
                        .left(px(0.0))
                        .right(px(0.0))
                        .bg(rgb(0xffffff))
                        .border_1()
                        .border_color(rgb(0xcccccc))
                        .rounded(px(4.0))
                        .shadow_lg()
                        .max_h(px(300.0))
                        .children(dropdown_items),
                )
            })
    }

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
            .gap_3()
            .bg(rgb(0xffffff))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xdddddd))
            .p_4()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x333333))
                    .mb_2()
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
                4 => self.render_step_5(cx),
                _ => div().child("Unknown step"),
            })
    }

    fn render_navigation_buttons(&mut self, total: usize, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .flex_row()
            .gap_3()
            .justify_between()
            .w_full()
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
        let has_curve = self.input_curve.is_some();
        let file_name = self
            .input_file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("");

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .child("Select your headphone measurement curve and target"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Input Curve:"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_2()
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(rgb(0x4a90e2))
                                    .bg(rgb(0x4a90e2))
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0x3a80d2)))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.load_file_curve(cx);
                                        }),
                                    )
                                    .child("📁 Load CSV File"),
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
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.load_demo_curve(cx);
                                        }),
                                    )
                                    .child("🔄 Load Demo Curve"),
                            ),
                    )
                    .child(if has_curve {
                        div()
                            .px_4()
                            .py_2()
                            .mt_2()
                            .rounded(px(4.0))
                            .bg(rgb(0xf0f8ff))
                            .border_1()
                            .border_color(rgb(0x50c878))
                            .text_sm()
                            .text_color(rgb(0x333333))
                            .child(if !file_name.is_empty() {
                                format!("✓ Loaded: {}", file_name)
                            } else {
                                "✓ Demo curve loaded".to_string()
                            })
                    } else {
                        div()
                            .px_4()
                            .py_2()
                            .mt_2()
                            .rounded(px(4.0))
                            .bg(rgb(0xf9f9f9))
                            .border_1()
                            .border_color(rgb(0xeeeeee))
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child("No curve loaded yet")
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Target Curve:"),
                    )
                    .child(self.render_target_dropdown(cx)),
            )
    }

    fn render_step_2(&mut self, _cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .w_full()
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .mb_4()
                    .child("Configure EQ parameters and run optimization"),
            )
            .child(
                // Embed the EQDesignComponent
                div()
                    .w_full()
                    .child(self.eq_design.clone()),
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
                                .child("View the optimized frequency response"),
                        )
                        .child(
                            // Render the actual plot component
                            div().w_full().child(self.plot_component.clone()),
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
                                    "ℹ {} filters optimized successfully!",
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
            .child("Run optimization in Step 2 to see results")
    }

    fn render_step_4(&mut self) -> Div {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .child("Configure audio interface and preview the EQ"),
            )
            .child(
                // Audio interface component
                div().w_full().child(self.audio_interface.clone()),
            )
            .child(
                // Audio player controls (placeholder)
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_4()
                    .rounded(px(8.0))
                    .bg(rgb(0xfafafa))
                    .border_1()
                    .border_color(rgb(0xdddddd))
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x333333))
                            .child("Audio Preview"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child("Audio playback with real-time EQ preview coming soon..."),
                    ),
            )
    }

    fn render_step_5(&mut self, cx: &mut Context<Self>) -> Div {
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
            .gap_6()
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .child("Review and save your EQ settings"),
            )
            .child(
                // Display filter parameters
                div().w_full().child(self.filter_display.clone()),
            )
            .child(
                // Also show the frequency plot for reference
                div().w_full().child(self.plot_component.clone()),
            )
            .child(
                // Export buttons
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .rounded(px(8.0))
                    .bg(rgb(0xfafafa))
                    .border_1()
                    .border_color(rgb(0xdddddd))
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x333333))
                            .mb_2()
                            .child("Export Options:"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_2()
                            .child(
                                div()
                                    .px_6()
                                    .py_3()
                                    .rounded(px(6.0))
                                    .bg(rgb(0x4a90e2))
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0x3a80d2)))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, _| {
                                            this.export_filters(ExportFormat::CamillaDSP);
                                        }),
                                    )
                                    .child("💾 Export CamillaDSP"),
                            )
                            .child(
                                div()
                                    .px_6()
                                    .py_3()
                                    .rounded(px(6.0))
                                    .bg(rgb(0x4a90e2))
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0x3a80d2)))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, _| {
                                            this.export_filters(ExportFormat::ParametricEQ);
                                        }),
                                    )
                                    .child("💾 Export Parametric EQ"),
                            )
                            .child(
                                div()
                                    .px_6()
                                    .py_3()
                                    .rounded(px(6.0))
                                    .bg(rgb(0x4a90e2))
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0x3a80d2)))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, _| {
                                            this.export_filters(ExportFormat::REW);
                                        }),
                                    )
                                    .child("💾 Export REW"),
                            ),
                    ),
            )
    }
}
