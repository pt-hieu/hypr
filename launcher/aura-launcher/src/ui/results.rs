use super::LauncherState;
use gtk4::gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Image, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
    SelectionMode,
};
use std::cell::RefCell;
use std::rc::Rc;

/// Build the results list
pub fn build_results(state: Rc<RefCell<LauncherState>>) -> ScrolledWindow {
    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .build();

    let list_box = ListBox::builder()
        .selection_mode(SelectionMode::Single)
        .build();

    list_box.add_css_class("launcher-results");

    // Populate initial results
    populate_results(&list_box, &state);

    // Connect to state changes via idle callback
    // This is a simple approach - in production you'd use proper signals
    let list_box_clone = list_box.clone();
    let state_clone = state.clone();

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        populate_results(&list_box_clone, &state_clone);
        gtk4::glib::ControlFlow::Continue
    });

    scrolled.set_child(Some(&list_box));
    scrolled
}

/// Populate the list with current filtered apps
fn populate_results(list_box: &ListBox, state: &Rc<RefCell<LauncherState>>) {
    let state = state.borrow();

    // Remove existing children
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    // Add filtered apps
    for (idx, app) in state.filtered_apps.iter().enumerate() {
        let row = create_result_row(app, &state, idx == state.selected_index);
        list_box.append(&row);
    }

    // Select the current row
    if let Some(row) = list_box.row_at_index(state.selected_index as i32) {
        list_box.select_row(Some(&row));
    }
}

/// Create a single result row
fn create_result_row(
    app: &aura_lib::DesktopApp,
    state: &LauncherState,
    is_selected: bool,
) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.add_css_class("launcher-result-row");

    if is_selected {
        row.add_css_class("selected");
    }

    let hbox = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Icon
    let icon_size = state.config.appearance.icon_size as i32;
    let image = if let Some(ref icon_name) = app.icon {
        if let Some(icon_path) = state.icon_cache.get(icon_name) {
            if let Ok(pixbuf) = Pixbuf::from_file_at_scale(&icon_path, icon_size, icon_size, true) {
                Image::from_pixbuf(Some(&pixbuf))
            } else {
                Image::from_icon_name("application-x-executable")
            }
        } else {
            Image::from_icon_name("application-x-executable")
        }
    } else {
        Image::from_icon_name("application-x-executable")
    };

    image.set_pixel_size(icon_size);
    image.add_css_class("launcher-result-icon");
    hbox.append(&image);

    // Text container
    let text_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(2)
        .hexpand(true)
        .build();

    // App name
    let name_label = Label::builder()
        .label(&app.name)
        .halign(gtk4::Align::Start)
        .build();
    name_label.add_css_class("launcher-result-name");
    text_box.append(&name_label);

    // Description (if available)
    if let Some(ref desc) = app.description {
        let desc_label = Label::builder()
            .label(desc)
            .halign(gtk4::Align::Start)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .build();
        desc_label.add_css_class("launcher-result-description");
        text_box.append(&desc_label);
    }

    hbox.append(&text_box);
    row.set_child(Some(&hbox));

    row
}
