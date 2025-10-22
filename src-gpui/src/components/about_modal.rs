use gpui::*;

pub struct AboutModal {
    is_visible: bool,
}

impl AboutModal {
    pub fn new() -> Self {
        Self { is_visible: false }
    }

    pub fn show(&mut self, cx: &mut Context<Self>) {
        self.is_visible = true;
        cx.notify();
    }

    pub fn hide(&mut self, cx: &mut Context<Self>) {
        self.is_visible = false;
        cx.notify();
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}

impl Render for AboutModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        // Modal overlay
        div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .right(px(0.0))
            .bottom(px(0.0))
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(0x00000080)) // Semi-transparent black
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.hide(cx);
                }),
            )
            .child(
                // Modal content
                div()
                    .w(px(500.0))
                    .bg(rgb(0xffffff))
                    .rounded(px(12.0))
                    .shadow_lg()
                    .border_1()
                    .border_color(rgb(0xdddddd))
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(
                        // Header
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p_6()
                            .border_b_1()
                            .border_color(rgb(0xeeeeee))
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x333333))
                                    .child("About SotF"),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .text_2xl()
                                    .text_color(rgb(0x999999))
                                    .hover(|s| s.text_color(rgb(0x333333)))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.hide(cx);
                                        }),
                                    )
                                    .child("×"),
                            ),
                    )
                    .child(
                        // Body
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .p_6()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x333333))
                                            .child("AutoEQ - Sound of the Future"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Version 0.1.1"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x666666))
                                    .child(
                                        "A native Rust application for optimizing speaker and headphone \
                                        equalization based on measurements using differential evolution \
                                        and metaheuristics algorithms.",
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x333333))
                                            .child("Author"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Pierre Aubert <pierre@spinorama.org>"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x333333))
                                            .child("License"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("GNU General Public License v3.0"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x333333))
                                            .child("Repository"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x4a90e2))
                                            .child("github.com/pierreaubert/autoEQ-app"),
                                    ),
                            ),
                    )
                    .child(
                        // Footer
                        div()
                            .flex()
                            .flex_row()
                            .justify_end()
                            .p_4()
                            .border_t_1()
                            .border_color(rgb(0xeeeeee))
                            .child(
                                div()
                                    .px_6()
                                    .py_2()
                                    .rounded(px(6.0))
                                    .bg(rgb(0x4a90e2))
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0x3a80d2)))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.hide(cx);
                                        }),
                                    )
                                    .child("Close"),
                            ),
                    ),
            )
    }
}
