// GPUI main application entry point
#![recursion_limit = "256"]

use gpui::*;

mod app_setup;
mod components;
mod design;
mod theme;
mod workflows;

fn main() {
    env_logger::init();

    Application::new().run(|cx: &mut App| {
        // Setup menu bar and keyboard shortcuts
        app_setup::init_app(cx);

        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|cx| MainView::new(cx))
        })
        .expect("failed to open window");
        
        // Note: Workflow switching via menu requires more complex GPUI integration
        // For now, these are handled via the home screen buttons
        cx.on_action(|_: &app_setup::SelectHeadphone, _cx| {
            log::info!("Headphone workflow selected from menu - use home screen to switch");
        });
        
        cx.on_action(|_: &app_setup::SelectSpeaker, _cx| {
            log::info!("Speaker workflow selected from menu - use home screen to switch");
        });
        
        cx.on_action(|_: &app_setup::SelectRoom, _cx| {
            log::info!("Room workflow selected from menu - use home screen to switch");
        });
        
        cx.on_action(|_: &app_setup::ShowAbout, _cx| {
            log::info!("About dialog - implementation requires event bus or global state");
            // TODO: Implement about dialog with proper GPUI patterns
        });

        cx.activate(true);
    });
}

pub struct MainView {
    screen: Screen,
    headphone_workflow: Option<Entity<workflows::headphone::HeadphoneWorkflow>>,
    speaker_workflow: Option<Entity<workflows::speaker::SpeakerWorkflow>>,
    room_workflow: Option<Entity<workflows::room::RoomWorkflow>>,
    about_modal: Entity<components::about_modal::AboutModal>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Headphone,
    Speaker,
    Room,
}

impl MainView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let about_modal = cx.new(|_cx| components::about_modal::AboutModal::new());
        
        Self {
            screen: Screen::Home,
            headphone_workflow: None,
            speaker_workflow: None,
            room_workflow: None,
            about_modal,
        }
    }

    fn go_to(&mut self, screen: Screen, cx: &mut Context<Self>) {
        self.screen = screen;
        cx.notify();
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let main_content = match self.screen {
            Screen::Home => div().child(self.render_home(cx)),
            Screen::Headphone => {
                // Create workflow if it doesn't exist
                if self.headphone_workflow.is_none() {
                    self.headphone_workflow =
                        Some(cx.new(|cx| workflows::headphone::HeadphoneWorkflow::new(cx)));
                }
                let workflow = self.headphone_workflow.as_ref().unwrap().clone();
                div().child(self.render_workflow_with_entity("Headphone", workflow, cx))
            }
            Screen::Speaker => {
                // Create workflow if it doesn't exist
                if self.speaker_workflow.is_none() {
                    self.speaker_workflow =
                        Some(cx.new(|_cx| workflows::speaker::SpeakerWorkflow::new()));
                }
                let workflow = self.speaker_workflow.as_ref().unwrap().clone();
                div().child(self.render_workflow_with_speaker("Speaker", workflow, cx))
            }
            Screen::Room => {
                // Create workflow if it doesn't exist
                if self.room_workflow.is_none() {
                    self.room_workflow = Some(cx.new(|_cx| workflows::room::RoomWorkflow::new()));
                }
                let workflow = self.room_workflow.as_ref().unwrap().clone();
                div().child(self.render_workflow_with_room("Room", workflow, cx))
            }
        };
        
        // Render main content with about modal overlay
        div()
            .relative()
            .w_full()
            .h_full()
            .child(main_content)
            .child(self.about_modal.clone())
    }
}

impl MainView {
    fn render_home(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .gap_8()
            .bg(design::colors::bg_primary())
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(design::colors::text_primary())
                    .child("AutoEQ - Choose Your Workflow"),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_8()
                    .child(self.workflow_button("🎧\nHeadphone".to_string(), Screen::Headphone, cx))
                    .child(self.workflow_button("🔊\nSpeaker".to_string(), Screen::Speaker, cx))
                    .child(self.workflow_button("🏠\nRoom".to_string(), Screen::Room, cx)),
            )
    }

    fn workflow_button(
        &mut self,
        label: String,
        target: Screen,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_current = self.screen == target;

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w(px(200.0))
            .h(px(200.0))
            .rounded(px(12.0))
            .bg(if is_current {
                design::colors::button_primary()
            } else {
                design::colors::bg_secondary()
            })
            .border_1()
            .border_color(design::colors::border())
            .shadow_lg()
            .text_xl()
            .text_color(if is_current {
                white()
            } else {
                design::colors::text_primary()
            })
            .cursor_pointer()
            .hover(|s| s.bg(design::colors::button_primary_hover()))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.go_to(target, cx);
                }),
            )
            .child(label)
    }

    fn render_workflow(
        &mut self,
        title: &str,
        content: impl IntoElement,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(design::colors::bg_primary())
            .child(
                // Header with back button
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_4()
                    .p_4()
                    .border_b_1()
                    .border_color(design::colors::border())
                    .bg(design::colors::bg_secondary())
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded(px(6.0))
                            .bg(design::colors::button_primary())
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.bg(design::colors::button_primary_hover()))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.go_to(Screen::Home, cx);
                                }),
                            )
                            .child("← Back"),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(design::colors::text_primary())
                            .child(format!("{} Workflow", title)),
                    ),
            )
            .child(
                // Content area
                div().flex_1().p_6().child(content),
            )
    }

    fn render_workflow_with_entity(
        &mut self,
        title: &str,
        entity: Entity<workflows::headphone::HeadphoneWorkflow>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(design::colors::bg_primary())
            .child(
                // Header with back button
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_4()
                    .p_4()
                    .border_b_1()
                    .border_color(design::colors::border())
                    .bg(design::colors::bg_secondary())
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded(px(6.0))
                            .bg(design::colors::button_primary())
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.bg(design::colors::button_primary_hover()))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.go_to(Screen::Home, cx);
                                }),
                            )
                            .child("← Back"),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(design::colors::text_primary())
                            .child(format!("{} Workflow", title)),
                    ),
            )
            .child(
                // Content area
                div().flex_1().p_6().child(entity),
            )
    }

    fn render_workflow_with_speaker(
        &mut self,
        title: &str,
        entity: Entity<workflows::speaker::SpeakerWorkflow>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(design::colors::bg_primary())
            .child(
                // Header with back button
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_4()
                    .p_4()
                    .border_b_1()
                    .border_color(design::colors::border())
                    .bg(design::colors::bg_secondary())
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded(px(6.0))
                            .bg(design::colors::button_primary())
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.bg(design::colors::button_primary_hover()))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.go_to(Screen::Home, cx);
                                }),
                            )
                            .child("← Back"),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(design::colors::text_primary())
                            .child(format!("{} Workflow", title)),
                    ),
            )
            .child(
                // Content area
                div().flex_1().p_6().child(entity),
            )
    }

    fn render_workflow_with_room(
        &mut self,
        title: &str,
        entity: Entity<workflows::room::RoomWorkflow>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(design::colors::bg_primary())
            .child(
                // Header with back button
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_4()
                    .p_4()
                    .border_b_1()
                    .border_color(design::colors::border())
                    .bg(design::colors::bg_secondary())
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded(px(6.0))
                            .bg(design::colors::button_primary())
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.bg(design::colors::button_primary_hover()))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.go_to(Screen::Home, cx);
                                }),
                            )
                            .child("← Back"),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(design::colors::text_primary())
                            .child(format!("{} Workflow", title)),
                    ),
            )
            .child(
                // Content area
                div().flex_1().p_6().child(entity),
            )
    }
}
