use super::LauncherState;
use gtk4::gdk::Key;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Entry, EventControllerKey};
use std::cell::RefCell;
use std::rc::Rc;

/// Build the search input field
pub fn build_input(
    window: &ApplicationWindow,
    state: Rc<RefCell<LauncherState>>,
) -> Entry {
    let input = Entry::builder()
        .placeholder_text("Search applications...")
        .hexpand(true)
        .build();

    input.add_css_class("launcher-input");
    input.set_widget_name("launcher-input");

    // Handle text changes
    let state_clone = state.clone();
    input.connect_changed(move |entry| {
        let query = entry.text();
        let mut state = state_clone.borrow_mut();
        state.filter(&query);

        // Trigger UI update by emitting a signal
        // The results list will pick this up
        drop(state);
    });

    // Handle keyboard navigation
    let key_controller = EventControllerKey::new();
    let window_clone = window.clone();
    let state_clone = state.clone();

    key_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            Key::Escape => {
                // Close the launcher
                window_clone.close();
                gtk4::glib::Propagation::Stop
            }
            Key::Return | Key::KP_Enter => {
                // Launch selected app
                let mut state = state_clone.borrow_mut();
                if state.launch_selected() {
                    drop(state);
                    window_clone.close();
                }
                gtk4::glib::Propagation::Stop
            }
            Key::Up => {
                let mut state = state_clone.borrow_mut();
                state.select_prev();
                gtk4::glib::Propagation::Stop
            }
            Key::Down => {
                let mut state = state_clone.borrow_mut();
                state.select_next();
                gtk4::glib::Propagation::Stop
            }
            Key::Tab => {
                // Tab moves down, Shift+Tab moves up
                let mut state = state_clone.borrow_mut();
                state.select_next();
                gtk4::glib::Propagation::Stop
            }
            _ => gtk4::glib::Propagation::Proceed,
        }
    });

    input.add_controller(key_controller);

    // Focus the input on show
    input.grab_focus();

    input
}
