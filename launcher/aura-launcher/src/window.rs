use crate::config::Config;
use crate::ui::{build_ui, LauncherState};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;

pub struct LauncherWindow {
    window: ApplicationWindow,
}

impl LauncherWindow {
    pub fn new(app: &Application, config: &Config) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Aura Launcher")
            .default_width(config.launcher.width)
            .default_height(config.launcher.height)
            .build();

        // Initialize layer shell
        window.init_layer_shell();

        // Set layer to top (above normal windows)
        window.set_layer(Layer::Top);

        // Anchor to top, horizontally centered
        window.set_anchor(Edge::Top, true);

        // Set margin from top
        window.set_margin(Edge::Top, 100);

        // Request exclusive keyboard focus
        window.set_keyboard_mode(KeyboardMode::Exclusive);

        // Create shared state
        let state = Rc::new(RefCell::new(LauncherState::new(config)));

        // Build the UI
        let content = build_ui(&window, state);
        window.set_child(Some(&content));

        // Add CSS classes
        window.add_css_class("launcher-window");

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
