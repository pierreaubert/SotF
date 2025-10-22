// Example: Routing Matrix Component
//
// This example demonstrates the RoutingMatrixComponent, which allows
// visual configuration of audio channel routing.
//
// Run with: cargo run --example routing_matrix

use gpui::*;

// Import components from the main crate
use autoeq_gpui::app_setup;
use autoeq_gpui::components::routing_matrix::RoutingMatrixComponent;
use autoeq_gpui::design::{colors, fonts, spacing, RADIUS};

fn main() {
    env_logger::init();

    Application::new().run(|cx: &mut App| {
        // Setup menu bar and keyboard shortcuts
        app_setup::init_app(cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point {
                        x: px(100.0),
                        y: px(100.0),
                    },
                    size: Size {
                        width: px(800.0),
                        height: px(600.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("Routing Matrix Example".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| ExampleView::new(cx)),
        )
        .expect("failed to open window");

        cx.activate(true);
    });
}

struct ExampleView {
    routing_matrix_2ch: Entity<RoutingMatrixComponent>,
    routing_matrix_4ch: Entity<RoutingMatrixComponent>,
    routing_matrix_8ch: Entity<RoutingMatrixComponent>,
}

impl ExampleView {
    fn new(cx: &mut Context<Self>) -> Self {
        let routing_matrix_2ch = cx.new(|_cx| RoutingMatrixComponent::new(2));
        let routing_matrix_4ch = cx.new(|_cx| RoutingMatrixComponent::new(4));
        let routing_matrix_8ch = cx.new(|_cx| RoutingMatrixComponent::new(8));

        Self {
            routing_matrix_2ch,
            routing_matrix_4ch,
            routing_matrix_8ch,
        }
    }
}

impl Render for ExampleView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(colors::bg_primary())
            .p(spacing::XL)
            .child(
                // Header
                div()
                    .flex()
                    .flex_col()
                    .gap(spacing::MD)
                    .mb(spacing::XL)
                    .child(
                        div()
                            .text_size(fonts::SIZE_2XL)
                            .font_weight(FontWeight::BOLD)
                            .text_color(colors::text_primary())
                            .child("Routing Matrix Component")
                    )
                    .child(
                        div()
                            .text_size(fonts::SIZE_BASE)
                            .text_color(colors::text_secondary())
                            .child("Configure audio channel routing with interactive visual grids.")
                    )
                    .child(
                        div()
                            .text_size(fonts::SIZE_SM)
                            .text_color(colors::text_secondary())
                            .child("Click on matrix cells to route logical channels to physical channels. The system automatically swaps channels to maintain valid routing.")
                    )
            )
            .child(
                // Configuration examples
                div()
                    .flex()
                    .flex_col()
                    .gap(spacing::LG)
                    .child(
                        // Stereo (2 channel)
                        div()
                            .p(spacing::LG)
                            .bg(colors::bg_secondary())
                            .rounded(RADIUS)
                            .border_1()
                            .border_color(colors::border())
                            .shadow_sm()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::MD)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .justify_between()
                                            .items_center()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(spacing::XS)
                                                    .child(
                                                        div()
                                                            .text_size(fonts::SIZE_LG)
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(colors::text_primary())
                                                            .child("Stereo (2 Channel)")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(fonts::SIZE_SM)
                                                            .text_color(colors::text_secondary())
                                                            .child("Common for headphones and basic speakers")
                                                    )
                                            )
                                            .child(
                                                self.render_button("Configure 2ch", cx, 2)
                                            )
                                    )
                                    .child(
                                        div()
                                            .text_size(fonts::SIZE_SM)
                                            .text_color(colors::text_secondary())
                                            .child("Channels: Left, Right")
                                    )
                            )
                    )
                    .child(
                        // Quadraphonic (4 channel)
                        div()
                            .p(spacing::LG)
                            .bg(colors::bg_secondary())
                            .rounded(RADIUS)
                            .border_1()
                            .border_color(colors::border())
                            .shadow_sm()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::MD)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .justify_between()
                                            .items_center()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(spacing::XS)
                                                    .child(
                                                        div()
                                                            .text_size(fonts::SIZE_LG)
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(colors::text_primary())
                                                            .child("Quadraphonic (4 Channel)")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(fonts::SIZE_SM)
                                                            .text_color(colors::text_secondary())
                                                            .child("Surround sound with subwoofer")
                                                    )
                                            )
                                            .child(
                                                self.render_button("Configure 4ch", cx, 4)
                                            )
                                    )
                                    .child(
                                        div()
                                            .text_size(fonts::SIZE_SM)
                                            .text_color(colors::text_secondary())
                                            .child("Channels: Left, Right, Center, Subwoofer")
                                    )
                            )
                    )
                    .child(
                        // 7.1 Surround (8 channel)
                        div()
                            .p(spacing::LG)
                            .bg(colors::bg_secondary())
                            .rounded(RADIUS)
                            .border_1()
                            .border_color(colors::border())
                            .shadow_sm()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(spacing::MD)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .justify_between()
                                            .items_center()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(spacing::XS)
                                                    .child(
                                                        div()
                                                            .text_size(fonts::SIZE_LG)
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(colors::text_primary())
                                                            .child("7.1 Surround (8 Channel)")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(fonts::SIZE_SM)
                                                            .text_color(colors::text_secondary())
                                                            .child("Full home theater setup")
                                                    )
                                            )
                                            .child(
                                                self.render_button("Configure 8ch", cx, 8)
                                            )
                                    )
                                    .child(
                                        div()
                                            .text_size(fonts::SIZE_SM)
                                            .text_color(colors::text_secondary())
                                            .child("Channels: Left, Right, Center, Sub, SR, SL, RR, RL")
                                    )
                            )
                    )
            )
            .child(
                // Info box
                div()
                    .mt(spacing::XL)
                    .p(spacing::MD)
                    .bg(colors::bg_accent())
                    .rounded(RADIUS)
                    .border_1()
                    .border_color(colors::border())
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(spacing::XS)
                            .child(
                                div()
                                    .text_size(fonts::SIZE_SM)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(colors::text_primary())
                                    .child("💡 How to use:")
                            )
                            .child(
                                div()
                                    .text_size(fonts::SIZE_SM)
                                    .text_color(colors::text_secondary())
                                    .child("• Click a button above to open the routing matrix for that channel configuration")
                            )
                            .child(
                                div()
                                    .text_size(fonts::SIZE_SM)
                                    .text_color(colors::text_secondary())
                                    .child("• In the matrix, rows represent logical channels and columns represent physical outputs")
                            )
                            .child(
                                div()
                                    .text_size(fonts::SIZE_SM)
                                    .text_color(colors::text_secondary())
                                    .child("• Click a cell to route that logical channel to the corresponding physical channel")
                            )
                            .child(
                                div()
                                    .text_size(fonts::SIZE_SM)
                                    .text_color(colors::text_secondary())
                                    .child("• The system will automatically swap if another channel is already routed to that output")
                            )
                            .child(
                                div()
                                    .text_size(fonts::SIZE_SM)
                                    .text_color(colors::text_secondary())
                                    .child("• Click 'Close' or press Escape to dismiss the matrix")
                            )
                    )
            )
            .child(
                // Routing matrix overlays (only visible when toggled)
                div()
                    .child(self.routing_matrix_2ch.clone())
                    .child(self.routing_matrix_4ch.clone())
                    .child(self.routing_matrix_8ch.clone())
            )
    }
}

impl ExampleView {
    fn render_button(&self, label: &str, cx: &mut Context<Self>, channels: usize) -> Div {
        let label = label.to_string();
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
                cx.listener(move |this, _, _, cx| match channels {
                    2 => this.routing_matrix_2ch.update(cx, |matrix, cx| {
                        matrix.toggle_visibility(cx);
                    }),
                    4 => this.routing_matrix_4ch.update(cx, |matrix, cx| {
                        matrix.toggle_visibility(cx);
                    }),
                    8 => this.routing_matrix_8ch.update(cx, |matrix, cx| {
                        matrix.toggle_visibility(cx);
                    }),
                    _ => {}
                }),
            )
            .child(label)
    }
}
