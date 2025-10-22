use crate::design::{colors, fonts, spacing, RADIUS};
/// Channel routing matrix component
/// Allows visual configuration of audio channel routing
use gpui::*;

pub struct RoutingMatrixComponent {
    channel_count: usize,
    routing: Vec<usize>, // routing[logical] = physical
    is_visible: bool,
}

impl RoutingMatrixComponent {
    pub fn new(channel_count: usize) -> Self {
        Self {
            channel_count,
            routing: (0..channel_count).collect(), // Identity routing by default
            is_visible: false,
        }
    }

    pub fn set_channel_count(&mut self, count: usize, cx: &mut Context<Self>) {
        self.channel_count = count;
        self.routing = (0..count).collect(); // Reset to identity
        cx.notify();
    }

    pub fn get_routing(&self) -> &[usize] {
        &self.routing
    }

    pub fn toggle_visibility(&mut self, cx: &mut Context<Self>) {
        self.is_visible = !self.is_visible;
        cx.notify();
    }

    fn set_routing(&mut self, logical: usize, physical: usize, cx: &mut Context<Self>) {
        // Swap if another logical channel is routed to this physical channel
        if let Some(other_logical) = self.routing.iter().position(|&p| p == physical) {
            if other_logical != logical {
                let temp = self.routing[logical];
                self.routing[logical] = physical;
                self.routing[other_logical] = temp;
            }
        } else {
            self.routing[logical] = physical;
        }
        cx.notify();
    }

    fn channel_name(index: usize) -> &'static str {
        match index {
            0 => "Left",
            1 => "Right",
            2 => "Center",
            3 => "Sub",
            4 => "SR",
            5 => "SL",
            6 => "RR",
            7 => "RL",
            _ => "Ch",
        }
    }
}

impl Render for RoutingMatrixComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        // Overlay
        div()
            .absolute()
            .top_0()
            .left_0()
            .w_full()
            .h_full()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.toggle_visibility(cx);
                }),
            )
            .child(
                div()
                    .bg(colors::bg_secondary())
                    .rounded(RADIUS)
                    .shadow_lg()
                    .p(spacing::LG)
                    .min_w(px(400.0))
                    .on_mouse_down(MouseButton::Left, |_, _, _| {
                        // Prevent click from closing the modal (handled by not propagating)
                    })
                    .child(
                        // Title
                        div()
                            .text_size(fonts::SIZE_LG)
                            .font_weight(FontWeight::BOLD)
                            .text_color(colors::text_primary())
                            .mb(spacing::MD)
                            .child("Channel Routing"),
                    )
                    .child(
                        // Matrix grid
                        self.render_matrix(cx),
                    )
                    .child(
                        // Close button
                        div().mt(spacing::MD).flex().justify_center().child(
                            div()
                                .px(spacing::LG)
                                .py(spacing::SM)
                                .rounded(RADIUS)
                                .bg(colors::button_primary())
                                .text_color(rgb(0xffffff))
                                .text_size(fonts::SIZE_BASE)
                                .font_weight(FontWeight::SEMIBOLD)
                                .cursor_pointer()
                                .hover(|s| s.bg(colors::button_primary_hover()))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.toggle_visibility(cx);
                                    }),
                                )
                                .child("Close"),
                        ),
                    ),
            )
    }
}

impl RoutingMatrixComponent {
    fn render_matrix(&self, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .flex_col()
            .gap(px(1.0))
            .child(
                // Header row with physical channel numbers
                div()
                    .flex()
                    .flex_row()
                    .gap(px(1.0))
                    .child(
                        // Corner cell
                        div()
                            .w(px(80.0))
                            .h(px(32.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(colors::bg_accent())
                            .border_1()
                            .border_color(colors::border())
                            .text_size(fonts::SIZE_SM)
                            .text_color(colors::text_secondary()),
                    )
                    .children((0..self.channel_count).map(|i| {
                        div()
                            .w(px(32.0))
                            .h(px(32.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(colors::bg_accent())
                            .border_1()
                            .border_color(colors::border())
                            .text_size(fonts::SIZE_SM)
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors::text_primary())
                            .child(format!("{}", i + 1))
                    })),
            )
            .children((0..self.channel_count).map(|logical| self.render_row(logical, cx)))
    }

    fn render_row(&self, logical: usize, cx: &mut Context<Self>) -> Div {
        let channel_name = Self::channel_name(logical);

        div()
            .flex()
            .flex_row()
            .gap(px(1.0))
            .child(
                // Row label (logical channel)
                div()
                    .w(px(80.0))
                    .h(px(32.0))
                    .flex()
                    .items_center()
                    .px(spacing::SM)
                    .bg(colors::bg_accent())
                    .border_1()
                    .border_color(colors::border())
                    .text_size(fonts::SIZE_SM)
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(colors::text_primary())
                    .child(if logical < 8 {
                        channel_name.to_string()
                    } else {
                        format!("{} {}", channel_name, logical + 1)
                    }),
            )
            .children((0..self.channel_count).map(|physical| {
                let is_active = self.routing[logical] == physical;

                div()
                    .w(px(32.0))
                    .h(px(32.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(if is_active {
                        colors::select_bg()
                    } else {
                        colors::bg_secondary()
                    })
                    .border_1()
                    .border_color(if is_active {
                        colors::select_border()
                    } else {
                        colors::border()
                    })
                    .text_size(fonts::SIZE_BASE)
                    .text_color(colors::text_primary())
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(if is_active {
                            colors::select_bg()
                        } else {
                            colors::hover_bg()
                        })
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.set_routing(logical, physical, cx);
                        }),
                    )
                    .child(if is_active { "×" } else { "" })
            }))
    }
}

/// Create a routing matrix icon button
pub fn routing_icon() -> Div {
    div()
        .w(px(16.0))
        .h(px(16.0))
        .flex()
        .items_center()
        .justify_center()
        .text_color(colors::text_secondary())
        .child(
            // Simple 3x3 grid icon using text
            div().text_size(px(10.0)).child("⊞"),
        )
}
