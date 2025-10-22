// Example: Audio Interface Component
//
// This example demonstrates the AudioInterfaceComponent, which displays
// available audio input and output devices on the system.
//
// Run with: cargo run --example audio_interface

use gpui::*;

// Import the component from the main crate
use autoeq_gpui::app_setup;
use autoeq_gpui::components::audio_interface::AudioInterfaceComponent;

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
                        width: px(700.0),
                        height: px(800.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("Audio Interface Example".into()),
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
    audio_interface: Entity<AudioInterfaceComponent>,
}

impl ExampleView {
    fn new(cx: &mut Context<Self>) -> Self {
        let audio_interface = cx.new(|cx| AudioInterfaceComponent::new(cx));

        Self { audio_interface }
    }
}

impl Render for ExampleView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0xf5f5f5))
            .p_8()
            .child(
                // Header
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .mb_6()
                    .child(
                        div()
                            .text_3xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x333333))
                            .child("Audio Interface Component"),
                    )
                    .child(div().text_base().text_color(rgb(0x666666)).child(
                        "This component displays all available audio devices on your system.",
                    ))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x999999))
                            .child("Click the Refresh button to re-scan for devices."),
                    ),
            )
            .child(
                // Component container
                div()
                    .flex_1()
                    .p_6()
                    .bg(rgb(0xffffff))
                    .rounded(px(8.0))
                    .shadow_lg()
                    .border_1()
                    .border_color(rgb(0xdddddd))
                    .child(self.audio_interface.clone()),
            )
    }
}
