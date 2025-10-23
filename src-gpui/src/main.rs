// GPUI main application entry point
#![recursion_limit = "256"]

use gpui::*;
use gpui::prelude::*;

mod app_setup;
mod components;
mod design;
mod app_theme;
mod workflows;

fn main() {
    env_logger::init();

    Application::new().run(|cx: &mut App| {
        // Setup menu bar and keyboard shortcuts
        app_setup::init_app(cx);

        let window = cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|cx| MainView::new(cx))
        })
        .expect("failed to open window");

        // Register theme action handlers that update the view
        let view_handle = window.clone();
        cx.on_action(move |_: &app_setup::SetThemeLight, cx| {
            app_theme::set_theme(app_theme::ThemeVariant::Light);
            _ = view_handle.update(cx, |view, _window, cx| view.on_theme_changed(cx));
        });

        let view_handle = window.clone();
        cx.on_action(move |_: &app_setup::SetThemeDark, cx| {
            app_theme::set_theme(app_theme::ThemeVariant::Dark);
            _ = view_handle.update(cx, |view, _window, cx| view.on_theme_changed(cx));
        });

        let view_handle = window.clone();
        cx.on_action(move |_: &app_setup::SetThemeBlue, cx| {
            app_theme::set_theme(app_theme::ThemeVariant::Blue);
            _ = view_handle.update(cx, |view, _window, cx| view.on_theme_changed(cx));
        });

        let view_handle = window.clone();
        cx.on_action(move |_: &app_setup::SetThemeHighContrast, cx| {
            app_theme::set_theme(app_theme::ThemeVariant::HighContrast);
            _ = view_handle.update(cx, |view, _window, cx| view.on_theme_changed(cx));
        });

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
    _theme_version: usize, // Incremented on theme changes to force re-render
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
            _theme_version: 0,
        }
    }

    pub fn on_theme_changed(&mut self, cx: &mut Context<Self>) {
        self._theme_version = self._theme_version.wrapping_add(1);
        cx.notify();
    }

    fn go_to(&mut self, screen: Screen, cx: &mut Context<Self>) {
        self.screen = screen;
        cx.notify();
    }
}

impl Render for MainView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_size = window.bounds().size;
        let main_content = match self.screen {
            Screen::Home => div()
		.child(self.render_home(window_size, cx)),
            Screen::Headphone => {
                // Create workflow if it doesn't exist
                if self.headphone_workflow.is_none() {
                    let workflow = cx.new(|cx| workflows::headphone::HeadphoneWorkflow::new(cx));
                    window.focus(&workflow.read(cx).focus_handle);
                    self.headphone_workflow = Some(workflow);
                }
                let workflow = self.headphone_workflow.as_ref().unwrap().clone();
                div()
		    .child(self.render_workflow_with_entity("Headphone", workflow, cx))
            }
            Screen::Speaker => {
                // Create workflow if it doesn't exist
                if self.speaker_workflow.is_none() {
                    let workflow = cx.new(|cx| workflows::speaker::SpeakerWorkflow::new(cx));
                    window.focus(&workflow.read(cx).focus_handle);
                    self.speaker_workflow = Some(workflow);
                }
                let workflow = self.speaker_workflow.as_ref().unwrap().clone();
                div()
		    .child(self.render_workflow_with_speaker("Speaker", workflow, cx))
            }
            Screen::Room => {
                // Create workflow if it doesn't exist
                if self.room_workflow.is_none() {
                    let workflow = cx.new(|cx| workflows::room::RoomWorkflow::new(cx));
                    window.focus(&workflow.read(cx).focus_handle);
                    self.room_workflow = Some(workflow);
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
    fn render_home(&mut self, window_size: Size<Pixels>, cx: &mut Context<Self>) -> impl IntoElement {
        // Responsive layout: use vertical if not enough horizontal space (< 900px)
        let window_width: f32 = window_size.width.into();
        let use_vertical = window_width < 900.0;

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .items_center()
            .justify_center()
            .bg(design::colors::bg_primary())
            .p_8()
            .child(
                // Centered content container
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_12()
                    .child(
                        // Title
                        div()
                            .text_3xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(design::colors::text_primary())
                            .child("Sound of the Future"),
                    )
                    .child(
                        // Buttons container - responsive layout
                        div()
                            .flex()
                            .when(use_vertical, |d| d.flex_col())
                            .when(!use_vertical, |d| d.flex_row())
                            .items_center()
                            .justify_around()
                            .gap_12()
                            .child(self.workflow_button("🎧\nHeadphone".to_string(), Screen::Headphone, cx))
                            .child(self.workflow_button("🔊\nSpeaker".to_string(), Screen::Speaker, cx))
                            .child(self.workflow_button("🏠\nRoom".to_string(), Screen::Room, cx)),
                    ),
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
            .w(px(280.0))
            .h(px(280.0))
            .rounded(px(20.0))
            .bg(if is_current {
                design::colors::button_primary()
            } else {
                design::colors::bg_secondary()
            })
            .border_2()
            .border_color(if is_current {
                design::colors::button_primary()
            } else {
                design::colors::border()
            })
            .shadow_lg()
            .text_3xl()
            .font_weight(FontWeight::BOLD)
            .text_center()
            .text_color(if is_current {
                white()
            } else {
                design::colors::text_primary()
            })
            .cursor_pointer()
            .hover(|style| {
                style.bg(if is_current {
                    design::colors::button_primary_hover()
                } else {
                    design::colors::hover_bg()
                }).border_color(design::colors::button_primary())
            })
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
        _title: &str,
        content: impl IntoElement,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(design::colors::bg_primary())
            .child(
                // Content area - full height
                div()
                    .flex_1()
                    .p_4()
                    .child(content),
            )
    }

    fn render_workflow_with_entity(
        &mut self,
        _title: &str,
        entity: Entity<workflows::headphone::HeadphoneWorkflow>,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(design::colors::bg_primary())
            .child(
                // Content area - full height
                div()
                    .w_full()
                    .h_full()
                    .child(entity),
            )
    }

    fn render_workflow_with_speaker(
        &mut self,
        _title: &str,
        entity: Entity<workflows::speaker::SpeakerWorkflow>,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(design::colors::bg_primary())
            .child(
                // Content area - full height
                div()
                    .w_full()
                    .h_full()
                    .child(entity),
            )
    }

    fn render_workflow_with_room(
        &mut self,
        _title: &str,
        entity: Entity<workflows::room::RoomWorkflow>,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(design::colors::bg_primary())
            .child(
                // Content area - full height
                div()
                    .w_full()
                    .h_full()
                    .child(entity),
            )
    }
}
