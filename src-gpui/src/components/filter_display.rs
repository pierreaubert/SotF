use autoeq_backend::export::FilterParam;
use gpui::*;

pub struct FilterDisplayComponent {
    filters: Vec<FilterParam>,
}

impl FilterDisplayComponent {
    pub fn new(filters: Vec<FilterParam>) -> Self {
        Self { filters }
    }

    pub fn set_filters(&mut self, filters: Vec<FilterParam>, cx: &mut Context<Self>) {
        self.filters = filters;
        cx.notify();
    }
}

impl Render for FilterDisplayComponent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .w_full()
            .child(
                // Header
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .pb_2()
                    .border_b_1()
                    .border_color(rgb(0xdddddd))
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x333333))
                            .child("Filter Parameters"),
                    )
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded(px(4.0))
                            .bg(rgb(0x4a90e2))
                            .text_color(rgb(0xffffff))
                            .text_sm()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x3a80d2)))
                            .child("📋 Copy"),
                    ),
            )
            .child(
                // Table
                div().w_full().child(self.render_table()),
            )
            .child(
                // Export buttons
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .pt_4()
                    .border_t_1()
                    .border_color(rgb(0xdddddd))
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(rgb(0xcccccc))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0xf5f5f5)))
                            .on_mouse_down(MouseButton::Left, |_, _, _| {
                                // Note: Can't capture self here due to Render constraints
                                // Export functionality moved to HeadphoneWorkflow
                            })
                            .child("Export CamillaDSP"),
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
                            .on_mouse_down(MouseButton::Left, |_, _, _| {
                                // Export functionality moved to HeadphoneWorkflow
                            })
                            .child("Export Parametric EQ"),
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
                            .on_mouse_down(MouseButton::Left, |_, _, _| {
                                // Export functionality moved to HeadphoneWorkflow
                            })
                            .child("Export REW"),
                    ),
            )
    }
}

impl FilterDisplayComponent {
    fn render_table(&self) -> Div {
        div()
            .w_full()
            .child(
                // Table header
                div()
                    .flex()
                    .flex_row()
                    .py_2()
                    .px_2()
                    .bg(rgb(0xf5f5f5))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_sm()
                    .child(div().w(px(60.0)).child("#"))
                    .child(div().w(px(100.0)).child("Type"))
                    .child(div().w(px(120.0)).child("Frequency (Hz)"))
                    .child(div().w(px(100.0)).child("Gain (dB)"))
                    .child(div().w(px(100.0)).child("Q")),
            )
            .children(
                self.filters
                    .iter()
                    .enumerate()
                    .map(|(i, filter)| self.render_filter_row(i + 1, filter)),
            )
    }

    fn render_filter_row(&self, index: usize, filter: &FilterParam) -> Div {
        let is_even = index % 2 == 0;

        div()
            .flex()
            .flex_row()
            .py_2()
            .px_2()
            .bg(if is_even {
                rgb(0xfafafa)
            } else {
                rgb(0xffffff)
            })
            .text_sm()
            .hover(|s| s.bg(rgb(0xe8f4ff)))
            .child(div().w(px(60.0)).child(index.to_string()))
            .child(div().w(px(100.0)).child(filter.filter_type.clone()))
            .child(div().w(px(120.0)).child(format!("{:.1}", filter.frequency)))
            .child(div().w(px(100.0)).child(format!("{:+.2}", filter.gain)))
            .child(div().w(px(100.0)).child(format!("{:.3}", filter.q)))
    }
}
