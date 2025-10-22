use gpui::*;

pub struct AudioCaptureComponent;

impl AudioCaptureComponent {
    pub fn new() -> Self {
        Self
    }
    pub fn demo_label() -> &'static str {
        "audio_capture"
    }
}

impl Render for AudioCaptureComponent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(div().child("Audio Capture"))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(div().child("[Record]"))
                    .child(div().child("[Stop]")),
            )
            .child(div().child("Level Meter (placeholder)"))
    }
}
