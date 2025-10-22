use gpui::*;

pub struct AudioPlayerComponent;

impl AudioPlayerComponent {
    pub fn new() -> Self {
        Self
    }
    pub fn demo_label() -> &'static str {
        "audio_player"
    }
}

impl Render for AudioPlayerComponent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(div().child("Audio Player"))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(div().child("[Play]"))
                    .child(div().child("[Pause]"))
                    .child(div().child("[Stop]")),
            )
            .child(div().child("Waveform (placeholder)"))
    }
}
