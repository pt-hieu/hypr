mod input;
mod results;

use crate::config::Config;
use aura_lib::{scan_applications, DesktopApp, FuzzyMatcher, History, IconCache};
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, Orientation};
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;

pub use input::build_input;
pub use results::build_results;

/// Shared application state
pub struct LauncherState {
    pub apps: Vec<DesktopApp>,
    pub history: History,
    pub matcher: FuzzyMatcher,
    pub icon_cache: IconCache,
    pub config: Config,
    pub filtered_apps: Vec<DesktopApp>,
    pub selected_index: usize,
}

impl LauncherState {
    pub fn new(config: &Config) -> Self {
        let apps = scan_applications();
        let history = History::load();
        let icon_cache = IconCache::new(
            200,
            config.appearance.icon_size,
            config.appearance.icon_theme.clone(),
        );

        // Initial filtered list (empty query = frecency sorted)
        let mut matcher = FuzzyMatcher::new();
        let results = matcher.match_apps("", &apps, &history, config.launcher.max_results);
        let filtered_apps: Vec<DesktopApp> = results.into_iter().map(|r| r.app).collect();

        Self {
            apps,
            history,
            matcher,
            icon_cache,
            config: config.clone(),
            filtered_apps,
            selected_index: 0,
        }
    }

    /// Update filtered apps based on query
    pub fn filter(&mut self, query: &str) {
        let results = self.matcher.match_apps(
            query,
            &self.apps,
            &self.history,
            self.config.launcher.max_results,
        );
        self.filtered_apps = results.into_iter().map(|r| r.app).collect();
        self.selected_index = 0;
    }

    /// Launch the currently selected app
    pub fn launch_selected(&mut self) -> bool {
        if let Some(app) = self.filtered_apps.get(self.selected_index) {
            let cmd = app.launch_command();
            log::info!("Launching: {}", cmd);

            // Record in history
            self.history.record_launch(&app.id);
            if let Err(e) = self.history.save() {
                log::warn!("Failed to save history: {}", e);
            }

            // Launch the application
            if let Err(e) = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .spawn()
            {
                log::error!("Failed to launch {}: {}", cmd, e);
                return false;
            }

            return true;
        }
        false
    }

    /// Move selection up
    pub fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if self.selected_index + 1 < self.filtered_apps.len() {
            self.selected_index += 1;
        }
    }
}

/// Build the main UI container
pub fn build_ui(
    window: &ApplicationWindow,
    state: Rc<RefCell<LauncherState>>,
) -> GtkBox {
    let container = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    container.add_css_class("launcher-container");

    // Build input field
    let input = build_input(window, state.clone());
    container.append(&input);

    // Build results list
    let results = build_results(state.clone());
    container.append(&results);

    container
}
