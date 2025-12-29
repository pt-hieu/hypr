use crate::config::Config;
use crate::window::LauncherWindow;
use gtk4::prelude::*;
use gtk4::{gdk, gio, Application, CssProvider};

const APP_ID: &str = "com.aura.launcher";

pub struct AuraLauncherApp {
    app: Application,
}

impl AuraLauncherApp {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id(APP_ID)
            .flags(gio::ApplicationFlags::FLAGS_NONE)
            .build();

        app.connect_startup(|_| {
            load_css();
        });

        app.connect_activate(|app| {
            let config = Config::load();
            let window = LauncherWindow::new(app, &config);
            window.present();
        });

        Self { app }
    }

    pub fn run(&self) -> i32 {
        self.app.run().into()
    }
}

fn load_css() {
    let provider = CssProvider::new();

    // Try to load user CSS first
    if let Some(css_path) = Config::css_path() {
        provider.load_from_path(&css_path);
    } else {
        // Fall back to embedded CSS
        provider.load_from_data(include_str!("../assets/style.css"));
    }

    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not get default display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
