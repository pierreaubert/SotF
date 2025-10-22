use gpui::*;

pub struct WorkflowNavigator {
    steps: Vec<String>,
    current: usize,
}

impl WorkflowNavigator {
    pub fn new(steps: Vec<String>) -> Self {
        Self { steps, current: 0 }
    }
}

impl Render for WorkflowNavigator {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Simple desktop layout for now
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .children(self.steps.iter().enumerate().map(|(i, label)| {
                        let active = i == self.current;
                        let style = if active { "[" } else { "(" };
                        let end_style = if active { "]" } else { ")" };
                        div().child(format!("{}{} {}{}", style, i + 1, label, end_style))
                    })),
            )
    }
}
