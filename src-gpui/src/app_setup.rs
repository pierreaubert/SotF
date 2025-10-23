/// Common application setup for main app and examples
/// Provides menu bar, keyboard shortcuts, and platform-specific functionality
use gpui::*;
use crate::app_theme::{self, ThemeVariant};

/// Setup menu bar with common actions
pub fn setup_menu(cx: &mut App) {
    cx.set_menus(vec![
        Menu {
            name: "SotF".into(),
            items: vec![
                MenuItem::action("Load...", LoadProject),
                MenuItem::action("Save...", SaveProject),
                MenuItem::separator(),
                MenuItem::action("Quit", Quit),
            ],
        },
        Menu {
            name: "Workflow".into(),
            items: vec![
                MenuItem::action("Headphone", SelectHeadphone),
                MenuItem::action("Speaker", SelectSpeaker),
                MenuItem::action("Room", SelectRoom),
            ],
        },
        Menu {
            name: "View".into(),
            items: vec![
                MenuItem::action("Light Theme", SetThemeLight),
                MenuItem::action("Dark Theme", SetThemeDark),
                MenuItem::action("Blue Theme", SetThemeBlue),
                MenuItem::action("High Contrast Theme", SetThemeHighContrast),
            ],
        },
        Menu {
            name: "Help".into(),
            items: vec![
                MenuItem::action("Help", ShowHelp),
                MenuItem::action("Support", ShowSupport),
                MenuItem::separator(),
                MenuItem::action("About", ShowAbout),
            ],
        },
    ]);
}

/// Setup global keyboard shortcuts
pub fn setup_keybindings(cx: &mut App) {
    // Platform-specific quit shortcut
    #[cfg(target_os = "macos")]
    {
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    }

    #[cfg(not(target_os = "macos"))]
    {
        cx.bind_keys([KeyBinding::new("ctrl-q", Quit, None)]);
    }
}

/// Complete app setup (menu + keybindings)
pub fn setup_app(cx: &mut App) {
    setup_menu(cx);
    setup_keybindings(cx);
}

// Define all actions
actions!(
    autoeq,
    [
        Quit,
        LoadProject,
        SaveProject,
        SelectHeadphone,
        SelectSpeaker,
        SelectRoom,
        ShowHelp,
        ShowSupport,
        ShowAbout,
        SetThemeLight,
        SetThemeDark,
        SetThemeBlue,
        SetThemeHighContrast,
    ]
);

impl Quit {
    pub fn new() -> Self {
        Self
    }
}

impl LoadProject {
    pub fn new() -> Self {
        Self
    }
}

impl SaveProject {
    pub fn new() -> Self {
        Self
    }
}

impl SelectHeadphone {
    pub fn new() -> Self {
        Self
    }
}

impl SelectSpeaker {
    pub fn new() -> Self {
        Self
    }
}

impl SelectRoom {
    pub fn new() -> Self {
        Self
    }
}

impl ShowHelp {
    pub fn new() -> Self {
        Self
    }
}

impl ShowSupport {
    pub fn new() -> Self {
        Self
    }
}

impl ShowAbout {
    pub fn new() -> Self {
        Self
    }
}

impl SetThemeLight {
    pub fn new() -> Self {
        Self
    }
}

impl SetThemeDark {
    pub fn new() -> Self {
        Self
    }
}

impl SetThemeBlue {
    pub fn new() -> Self {
        Self
    }
}

impl SetThemeHighContrast {
    pub fn new() -> Self {
        Self
    }
}

// Register all action handlers
pub fn register_action_handlers(cx: &mut App) {
    cx.on_action(|_action: &Quit, cx| {
        cx.quit();
    });

    // Note: Workflow and dialog actions will be handled by the views themselves
    // These are just placeholders for the menu items
    cx.on_action(|_action: &LoadProject, _cx| {
        log::info!("Load Project menu selected");
        // TODO: Implement project loading
    });

    cx.on_action(|_action: &SaveProject, _cx| {
        log::info!("Save Project menu selected");
        // TODO: Implement project saving
    });

    cx.on_action(|_action: &ShowHelp, _cx| {
        log::info!("Show Help menu selected");
        // TODO: Open help documentation
    });

    cx.on_action(|_action: &ShowSupport, _cx| {
        // Open GitHub issues URL
        let url = "https://github.com/pierreaubert/autoEQ-app/issues";
        if let Err(e) = open::that(url) {
            log::error!("Failed to open URL {}: {}", url, e);
        }
    });

    // Theme actions
    // Note: Theme changes take effect immediately and will be visible on next window update
    // Any interaction (mouse move, click, etc.) will trigger a repaint
    cx.on_action(|_action: &SetThemeLight, cx| {
        app_theme::set_theme(ThemeVariant::Light);
        log::info!("Theme changed to Light");
        // Force window update by scheduling a refresh
        cx.defer(|_| {});
    });

    cx.on_action(|_action: &SetThemeDark, cx| {
        app_theme::set_theme(ThemeVariant::Dark);
        log::info!("Theme changed to Dark");
        cx.defer(|_| {});
    });

    cx.on_action(|_action: &SetThemeBlue, cx| {
        app_theme::set_theme(ThemeVariant::Blue);
        log::info!("Theme changed to Blue");
        cx.defer(|_| {});
    });

    cx.on_action(|_action: &SetThemeHighContrast, cx| {
        app_theme::set_theme(ThemeVariant::HighContrast);
        log::info!("Theme changed to High Contrast");
        cx.defer(|_| {});
    });
}

/// Convenient initialization function that sets up everything
pub fn init_app(cx: &mut App) {
    register_action_handlers(cx);
    setup_app(cx);
}
