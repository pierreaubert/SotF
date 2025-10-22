use crate::design::{colors, components, fonts, spacing, StyledDiv, RADIUS};
use autoeq_backend::{
    optim::{run_optimization_internal, CancellationState, ProgressCallback, ProgressUpdate},
    CurveData, OptimizationParams, OptimizationResult,
};
use gpui::{prelude::FluentBuilder, *};
use std::sync::Arc;

/// Optimization status enum
#[derive(Clone, Debug)]
pub enum OptimizationStatus {
    Idle,
    Running,
    Success(OptimizationResult),
    Error(String),
}

/// Main EQ Design component with complete parameter management
pub struct EQDesignComponent {
    // === EQ Design Section Parameters ===
    // Loss & Curve
    loss: String,
    curve_name: String,

    // Filter Configuration
    num_filters: usize,
    sample_rate: f64,

    // dB Range
    min_db: f64,
    max_db: f64,

    // Q Range
    min_q: f64,
    max_q: f64,

    // Frequency Range
    min_freq: f64,
    max_freq: f64,

    // PEQ Model
    peq_model: String,

    // Spacing
    min_spacing_oct: f64,
    spacing_weight: f64,

    // === Optimization Fine Tuning Section Parameters ===
    // Algorithm
    algo: String,
    population: usize,
    maxeval: usize,

    // DE-specific parameters (conditionally shown)
    strategy: String,
    de_f: f64,
    de_cr: f64,
    adaptive_weight_f: f64,
    adaptive_weight_cr: f64,

    // Tolerance
    tolerance: f64,
    atolerance: f64,

    // Refinement
    refine: bool,
    local_algo: String,

    // Smoothing
    smooth: bool,
    smooth_n: usize,

    // === Status and Results ===
    optimization_status: OptimizationStatus,

    // === Progress Tracking ===
    current_iteration: usize,
    current_fitness: f64,
    cancellation_state: Option<Arc<CancellationState>>,

    // === Input Data ===
    // These will be set externally (from captured audio or loaded files)
    input_curve: Option<CurveData>,
    target_curve: Option<CurveData>,
}

impl EQDesignComponent {
    /// Create a new EQDesignComponent with default values
    /// These defaults match the TypeScript OPTIMIZATION_DEFAULTS from optimization-constants.ts
    pub fn new() -> Self {
        Self {
            // EQ Design Parameters - defaults
            loss: "speaker-flat".to_string(),
            curve_name: "Listening Window".to_string(),
            num_filters: 5,
            sample_rate: 48000.0,
            min_db: 1.0,
            max_db: 3.0,
            min_q: 1.0,
            max_q: 3.0,
            min_freq: 60.0,
            max_freq: 16000.0,
            peq_model: "pk".to_string(),
            min_spacing_oct: 0.5,
            spacing_weight: 20.0,

            // Optimization Fine Tuning Parameters - defaults
            algo: "autoeq:de".to_string(),
            population: 30,
            maxeval: 20000,
            strategy: "currenttobest1bin".to_string(),
            de_f: 0.8,
            de_cr: 0.9,
            adaptive_weight_f: 0.8,
            adaptive_weight_cr: 0.7,
            tolerance: 1e-3,
            atolerance: 1e-4,
            refine: false,
            local_algo: "cobyla".to_string(),
            smooth: true,
            smooth_n: 1,

            // Status and data
            optimization_status: OptimizationStatus::Idle,
            current_iteration: 0,
            current_fitness: 0.0,
            cancellation_state: None,
            input_curve: None,
            target_curve: None,
        }
    }

    /// Reset all parameters to default values
    pub fn reset_to_defaults(&mut self, cx: &mut Context<Self>) {
        *self = Self::new();
        cx.notify();
    }

    /// Cancel ongoing optimization
    fn cancel_optimization(&mut self, cx: &mut Context<Self>) {
        if let Some(cancellation) = &self.cancellation_state {
            cancellation.cancel();
            log::info!("[EQDesign] Optimization cancellation requested");
        }
        self.optimization_status = OptimizationStatus::Error("Cancelled by user".to_string());
        self.cancellation_state = None;
        cx.notify();
    }

    pub fn demo_label() -> &'static str {
        "eq_design"
    }
}

impl EQDesignComponent {
    /// Set input curve data
    pub fn set_input_curve(&mut self, curve: CurveData, cx: &mut Context<Self>) {
        self.input_curve = Some(curve);
        cx.notify();
    }

    /// Set target curve data
    pub fn set_target_curve(&mut self, curve: Option<CurveData>, cx: &mut Context<Self>) {
        self.target_curve = curve;
        cx.notify();
    }

    /// Get the current optimization status
    pub fn optimization_status(&self) -> &OptimizationStatus {
        &self.optimization_status
    }

    /// Build OptimizationParams from current state
    fn build_params(&self) -> Result<OptimizationParams, String> {
        // Validate input curve exists
        let input_curve = self
            .input_curve
            .clone()
            .ok_or("No input curve data available")?;

        Ok(OptimizationParams {
            num_filters: self.num_filters,
            curve_path: None,
            target_path: None,
            sample_rate: self.sample_rate,
            max_db: self.max_db,
            min_db: self.min_db,
            max_q: self.max_q,
            min_q: self.min_q,
            min_freq: self.min_freq,
            max_freq: self.max_freq,
            speaker: None,
            version: None,
            measurement: None,
            curve_name: self.curve_name.clone(),
            algo: self.algo.clone(),
            population: self.population,
            maxeval: self.maxeval,
            refine: self.refine,
            local_algo: self.local_algo.clone(),
            min_spacing_oct: self.min_spacing_oct,
            spacing_weight: self.spacing_weight,
            smooth: self.smooth,
            smooth_n: self.smooth_n,
            loss: self.loss.clone(),
            peq_model: Some(self.peq_model.clone()),
            strategy: Some(self.strategy.clone()),
            de_f: Some(self.de_f),
            de_cr: Some(self.de_cr),
            adaptive_weight_f: Some(self.adaptive_weight_f),
            adaptive_weight_cr: Some(self.adaptive_weight_cr),
            tolerance: Some(self.tolerance),
            atolerance: Some(self.atolerance),
            captured_frequencies: Some(input_curve.freq),
            captured_magnitudes: Some(input_curve.spl),
            target_frequencies: self.target_curve.as_ref().map(|t| t.freq.clone()),
            target_magnitudes: self.target_curve.as_ref().map(|t| t.spl.clone()),
        })
    }

    /// Validate parameters
    fn validate_parameters(&self) -> Result<(), String> {
        // Validate numeric ranges
        if self.num_filters < 1 || self.num_filters > 20 {
            return Err("Number of filters must be between 1 and 20".to_string());
        }
        if self.min_db >= self.max_db {
            return Err("Min dB must be less than Max dB".to_string());
        }
        if self.min_q >= self.max_q {
            return Err("Min Q must be less than Max Q".to_string());
        }
        if self.min_freq >= self.max_freq {
            return Err("Min frequency must be less than Max frequency".to_string());
        }
        if self.input_curve.is_none() {
            return Err("No input curve data available".to_string());
        }
        Ok(())
    }

    /// Submit optimization
    fn submit_optimization(&mut self, cx: &mut Context<Self>) {
        // Validate parameters
        if let Err(error) = self.validate_parameters() {
            self.optimization_status = OptimizationStatus::Error(error);
            cx.notify();
            return;
        }

        // Build params
        let params = match self.build_params() {
            Ok(p) => p,
            Err(e) => {
                self.optimization_status = OptimizationStatus::Error(e);
                cx.notify();
                return;
            }
        };

        // Create cancellation state
        let cancellation_state = Arc::new(CancellationState::new());
        self.cancellation_state = Some(Arc::clone(&cancellation_state));

        // Update status to Running
        self.optimization_status = OptimizationStatus::Running;
        self.current_iteration = 0;
        self.current_fitness = 0.0;
        cx.notify();

        // Create progress callback
        struct UIProgressCallback {
            entity: WeakEntity<EQDesignComponent>,
        }

        impl ProgressCallback for UIProgressCallback {
            fn on_progress(&self, _update: ProgressUpdate) -> bool {
                // Note: UI updates from background threads are not supported in this GPUI version
                // Progress updates will be handled differently in a future version
                // For now, we'll just return true to continue optimization
                true // Continue optimization
            }
        }

        let progress_callback = Arc::new(UIProgressCallback {
            entity: cx.weak_entity(),
        });

        // Spawn async task to run optimization
        // Note: For this version, we'll run optimization synchronously to avoid complex async context handling
        // A future version can improve this with proper async/await patterns
        log::info!("[EQDesign] Starting optimization (blocking UI temporarily)");
        
        // Run in blocking context
        let result = cx.background_executor().block(run_optimization_internal(
            params,
            progress_callback,
            cancellation_state,
        ));
        
        // Update component with result
        self.cancellation_state = None;
        match result {
            Ok(opt_result) => {
                log::info!("[EQDesign] Optimization completed successfully");
                self.optimization_status = OptimizationStatus::Success(opt_result);
            }
            Err(e) => {
                log::error!("[EQDesign] Optimization failed: {}", e);
                self.optimization_status = OptimizationStatus::Error(e.to_string());
            }
        }
        cx.notify();
    }

    /// Render EQ Design section
    fn render_eq_design_section(&self, _cx: &mut Context<Self>) -> Div {
        div()
            .section_group()
            .child(components::section_header("EQ Design"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(spacing::SM)
                    // Loss and Curve Name row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Loss Function"))
                                    .child(
                                        div()
                                            .px(spacing::MD)
                                            .py(spacing::SM)
                                            .rounded(RADIUS)
                                            .border_1()
                                            .border_color(colors::border())
                                            .bg(colors::bg_secondary())
                                            .child(self.loss.clone()),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Curve"))
                                    .child(
                                        div()
                                            .px(spacing::MD)
                                            .py(spacing::SM)
                                            .rounded(RADIUS)
                                            .border_1()
                                            .border_color(colors::border())
                                            .bg(colors::bg_secondary())
                                            .child(self.curve_name.clone()),
                                    ),
                            ),
                    )
                    // Filters and Sample Rate row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Filters"))
                                    .child(
                                        div()
                                            .px(spacing::MD)
                                            .py(spacing::SM)
                                            .rounded(RADIUS)
                                            .border_1()
                                            .border_color(colors::border())
                                            .bg(colors::bg_secondary())
                                            .child(self.num_filters.to_string()),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Sample Rate"))
                                    .child(
                                        div()
                                            .px(spacing::MD)
                                            .py(spacing::SM)
                                            .rounded(RADIUS)
                                            .border_1()
                                            .border_color(colors::border())
                                            .bg(colors::bg_secondary())
                                            .child(self.sample_rate.to_string()),
                                    ),
                            ),
                    )
                    // dB Range row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Min dB"))
                                    .child(
                                        components::input_field().child(self.min_db.to_string()),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Max dB"))
                                    .child(
                                        components::input_field().child(self.max_db.to_string()),
                                    ),
                            ),
                    )
                    // Q Range row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Min Q"))
                                    .child(components::input_field().child(self.min_q.to_string())),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Max Q"))
                                    .child(components::input_field().child(self.max_q.to_string())),
                            ),
                    )
                    // Frequency Range row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Min Freq"))
                                    .child(
                                        components::input_field().child(self.min_freq.to_string()),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Max Freq"))
                                    .child(
                                        components::input_field().child(self.max_freq.to_string()),
                                    ),
                            ),
                    )
                    // PEQ Model
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(spacing::XS)
                            .child(components::label("PEQ Model"))
                            .child(components::input_field().child(self.peq_model.clone())),
                    )
                    // Spacing parameters
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Min Spacing (oct)"))
                                    .child(
                                        components::input_field()
                                            .child(self.min_spacing_oct.to_string()),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Spacing Weight"))
                                    .child(
                                        components::input_field()
                                            .child(self.spacing_weight.to_string()),
                                    ),
                            ),
                    ),
            )
    }

    /// Render Optimization Fine Tuning section
    fn render_optimization_section(&self, _cx: &mut Context<Self>) -> Div {
        let is_de_algo = self.algo == "autoeq:de";

        div()
            .section_group()
            .child(components::section_header("Optimization Fine Tuning"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(spacing::SM)
                    // Algorithm selection
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(spacing::XS)
                            .child(components::label("Algorithm"))
                            .child(components::input_field().child(self.algo.clone())),
                    )
                    // Population and MaxEval
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Population"))
                                    .child(
                                        components::input_field()
                                            .child(self.population.to_string()),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Max Eval"))
                                    .child(
                                        components::input_field().child(self.maxeval.to_string()),
                                    ),
                            ),
                    )
                    // DE-specific parameters (conditional)
                    .when(is_de_algo, |d| {
                        d.child(
                            gpui::div()
                                .flex()
                                .flex_col()
                                .gap(spacing::XS)
                                .child(components::label("Strategy"))
                                .child(components::input_field().child(self.strategy.clone())),
                        )
                        .child(
                            gpui::div()
                                .flex()
                                .flex_row()
                                .gap(spacing::MD)
                                .child(
                                    gpui::div()
                                        .flex()
                                        .flex_col()
                                        .gap(spacing::XS)
                                        .flex_1()
                                        .child(components::label("F (Mutation)"))
                                        .child(
                                            components::input_field().child(self.de_f.to_string()),
                                        ),
                                )
                                .child(
                                    gpui::div()
                                        .flex()
                                        .flex_col()
                                        .gap(spacing::XS)
                                        .flex_1()
                                        .child(components::label("CR (Recombination)"))
                                        .child(
                                            components::input_field().child(self.de_cr.to_string()),
                                        ),
                                ),
                        )
                    })
                    // Tolerance parameters
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Tolerance"))
                                    .child(
                                        components::input_field()
                                            .child(format!("{:.0e}", self.tolerance)),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::XS)
                                    .flex_1()
                                    .child(components::label("Abs Tolerance"))
                                    .child(
                                        components::input_field()
                                            .child(format!("{:.0e}", self.atolerance)),
                                    ),
                            ),
                    )
                    // Refine checkbox and local algo
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .items_center()
                            .child(
                                div()
                                    .px(spacing::SM)
                                    .py(spacing::XS)
                                    .child(if self.refine { "☑" } else { "☐" })
                                    .child(" Enable Refinement"),
                            )
                            .when(self.refine, |parent_div| {
                                parent_div.child(
                                    gpui::div()
                                        .flex()
                                        .flex_col()
                                        .gap(spacing::XS)
                                        .flex_1()
                                        .child(components::label("Local Optimizer"))
                                        .child(
                                            components::input_field()
                                                .child(self.local_algo.clone()),
                                        ),
                                )
                            }),
                    )
                    // Smooth checkbox and smooth_n
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(spacing::MD)
                            .items_center()
                            .child(
                                div()
                                    .px(spacing::SM)
                                    .py(spacing::XS)
                                    .child(if self.smooth { "☑" } else { "☐" })
                                    .child(" Enable Smoothing"),
                            )
                            .when(self.smooth, |parent_div| {
                                parent_div.child(
                                    gpui::div()
                                        .flex()
                                        .flex_col()
                                        .gap(spacing::XS)
                                        .w(px(150.0))
                                        .child(components::label("Smooth 1/N octave"))
                                        .child(
                                            components::input_field()
                                                .child(self.smooth_n.to_string()),
                                        ),
                                )
                            }),
                    ),
            )
    }

    /// Render status display
    fn render_status_display(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        match &self.optimization_status {
            OptimizationStatus::Idle => div()
                .px(spacing::MD)
                .py(spacing::SM)
                .bg(colors::bg_accent())
                .rounded(RADIUS)
                .text_color(colors::text_secondary())
                .child("Ready to optimize"),
            OptimizationStatus::Running => div()
                .flex()
                .flex_col()
                .gap(spacing::SM)
                .px(spacing::MD)
                .py(spacing::SM)
                .bg(colors::info())
                .text_color(rgb(0xffffff))
                .rounded(RADIUS)
                .child("⏳ Optimization running...")
                .when(self.current_iteration > 0, |d| {
                    d.child(format!(
                        "Iteration: {} | Fitness: {:.6}",
                        self.current_iteration, self.current_fitness
                    ))
                }),
            OptimizationStatus::Success(result) => div()
                .flex()
                .flex_col()
                .gap(spacing::SM)
                .px(spacing::MD)
                .py(spacing::SM)
                .bg(colors::success())
                .text_color(rgb(0xffffff))
                .rounded(RADIUS)
                .child("✅ Optimization successful!")
                .child(format!(
                    "Objective value: {:.6}",
                    result.objective_value.unwrap_or(0.0)
                ))
                .when_some(result.filter_params.as_ref(), |d, filters| {
                    d.child(
                        gpui::div()
                            .flex()
                            .flex_col()
                            .gap(spacing::XS)
                            .child("Filter Parameters:")
                            .child(format!("Raw parameters: {} values", filters.len()))
                            .child(
                                gpui::div()
                                    .text_size(fonts::SIZE_XS)
                                    .text_color(colors::text_secondary())
                                    .child("(Backend integration in progress - raw f64 values)"),
                            ),
                    )
                }),
            OptimizationStatus::Error(msg) => div()
                .px(spacing::MD)
                .py(spacing::SM)
                .bg(colors::danger())
                .text_color(rgb(0xffffff))
                .rounded(RADIUS)
                .child(format!("❌ Error: {}", msg)),
        }
    }
}

impl Render for EQDesignComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(spacing::MD)
            .w_full()
            .h_full()
            .p(spacing::MD)
            .bg(colors::bg_primary())
            // Status display
            .child(self.render_status_display(cx))
            // EQ Design section
            .child(self.render_eq_design_section(cx))
            // Optimization Fine Tuning section
            .child(self.render_optimization_section(cx))
            // Action buttons
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(spacing::MD)
                    .justify_between()
                    .mt(spacing::LG)
                    .child(
                        components::primary_button(
                            if matches!(self.optimization_status, OptimizationStatus::Running) {
                                "Running..."
                            } else {
                                "Run Optimization"
                            }
                        )
                        .when(!matches!(self.optimization_status, OptimizationStatus::Running), |btn| {
                            btn.on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.submit_optimization(cx);
                                }),
                            )
                        }),
                    )
                    .when(matches!(self.optimization_status, OptimizationStatus::Running), |parent| {
                        parent.child(
                            components::secondary_button("Cancel").on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.cancel_optimization(cx);
                                }),
                            ),
                        )
                    })
                    .when(!matches!(self.optimization_status, OptimizationStatus::Running), |parent| {
                        parent.child(
                            components::secondary_button("Reset to Defaults").on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.reset_to_defaults(cx);
                                }),
                            ),
                        )
                    }),
            )
    }
}
