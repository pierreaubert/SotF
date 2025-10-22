use autoeq_backend::{CurveData, PlotData};
use gpui::*;

pub struct FrequencyPlotComponent {
    input_curve: Option<CurveData>,
    target_curve: Option<CurveData>,
    optimized_curve: Option<CurveData>,
    // Additional plot data from backend
    filter_response: Option<PlotData>,
    width: f32,
    height: f32,
}

impl FrequencyPlotComponent {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            input_curve: None,
            target_curve: None,
            optimized_curve: None,
            filter_response: None,
            width,
            height,
        }
    }

    pub fn set_input_curve(&mut self, curve: CurveData, cx: &mut Context<Self>) {
        self.input_curve = Some(curve);
        cx.notify();
    }

    pub fn set_target_curve(&mut self, curve: CurveData, cx: &mut Context<Self>) {
        self.target_curve = Some(curve);
        cx.notify();
    }

    pub fn set_optimized_curve(&mut self, curve: CurveData, cx: &mut Context<Self>) {
        self.optimized_curve = Some(curve);
        cx.notify();
    }

    pub fn clear_target_curve(&mut self, cx: &mut Context<Self>) {
        self.target_curve = None;
        cx.notify();
    }

    pub fn set_filter_response(&mut self, plot_data: PlotData, cx: &mut Context<Self>) {
        self.filter_response = Some(plot_data);
        cx.notify();
    }

    fn compute_bounds(&self) -> (f64, f64, f64, f64) {
        let mut min_freq = 20.0;
        let mut max_freq = 20000.0;
        let mut min_mag = -20.0;
        let mut max_mag = 20.0;

        let curves = [&self.input_curve, &self.target_curve, &self.optimized_curve];

        for curve_opt in curves.iter() {
            if let Some(curve) = curve_opt {
                if !curve.freq.is_empty() {
                    let curve_min_freq = *curve
                        .freq
                        .iter()
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap();
                    let curve_max_freq = *curve
                        .freq
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap();
                    min_freq = f64::min(min_freq, curve_min_freq);
                    max_freq = f64::max(max_freq, curve_max_freq);
                }
                if !curve.spl.is_empty() {
                    let curve_min_mag = *curve
                        .spl
                        .iter()
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap();
                    let curve_max_mag = *curve
                        .spl
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap();
                    min_mag = f64::min(min_mag, curve_min_mag);
                    max_mag = f64::max(max_mag, curve_max_mag);
                }
            }
        }

        // Add padding
        let mag_range = max_mag - min_mag;
        min_mag -= mag_range * 0.1;
        max_mag += mag_range * 0.1;

        (min_freq, max_freq, min_mag, max_mag)
    }
}

impl Render for FrequencyPlotComponent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (min_freq, max_freq, min_mag, max_mag) = self.compute_bounds();

        div()
            .flex()
            .flex_col()
            .gap_2()
            .w(px(self.width))
            .h(px(self.height))
            .child(
                // Title
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x333333))
                    .child("Frequency Response"),
            )
            .child(
                // Legend
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .text_sm()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_1()
                            .items_center()
                            .child(div().w(px(20.0)).h(px(2.0)).bg(rgb(0xff6b6b)))
                            .child("Input"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_1()
                            .items_center()
                            .child(div().w(px(20.0)).h(px(2.0)).bg(rgb(0x4ecdc4)))
                            .child("Target"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_1()
                            .items_center()
                            .child(div().w(px(20.0)).h(px(2.0)).bg(rgb(0x95e1d3)))
                            .child("Optimized"),
                    ),
            )
            .child(
                // Plot area
                div()
                    .flex_1()
                    .w_full()
                    .rounded(px(4.0))
                    .border_1()
                    .border_color(rgb(0xdddddd))
                    .bg(rgb(0xffffff))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(div().text_color(rgb(0x999999)).child(format!(
                        "[Plot: {:.0}Hz - {:.0}Hz, {:.1}dB - {:.1}dB]",
                        min_freq, max_freq, min_mag, max_mag
                    ))),
            )
            .child(
                // X-axis label
                div()
                    .text_center()
                    .text_sm()
                    .text_color(rgb(0x666666))
                    .child("Frequency (Hz)"),
            )
    }
}
