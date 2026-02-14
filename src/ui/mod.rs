pub mod header;
pub mod list;
pub mod search_bar;

use std::{cell::RefCell, rc::Rc};

use crate::service::{cliboard_history::IClipboardHistory, cliboard_provider::IClipboardProvider};
use gtk::{
    Orientation,
    gdk::{self, Key},
    gio,
    glib::{self, object::ObjectExt},
    prelude::*,
};
use libadwaita as adw;

use crate::service::{
    cliboard_monitor::{ClipboardMonitor, IClipboardMonitor},
    cliphist_provider::CliphistProvider,
};

pub fn build_ui(app: &adw::Application, display: &gdk::Display) {
    register_styles(display);
    let (header_bar, search_button) = header::build();

    let content = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    content.append(&header_bar);

    let (search_bar, search_entry) = search_bar::build();

    search_button
        .bind_property("active", &search_bar, "search-mode-enabled")
        .sync_create()
        .bidirectional()
        .build();

    content.append(&search_bar);

    let clipboard_monitor = ClipboardMonitor::new(display);
    let history = clipboard_monitor.history();
    let current_clipboard = Rc::new(RefCell::new(None));

    let list_view = list::build(history.clone(), display, current_clipboard.clone());

    list::setup_search(&list_view, &search_entry);
    list::focus_list(&list_view);

    let provider = CliphistProvider;
    let display_clone = display.clone();

    glib::MainContext::default().spawn_local(glib::clone!(
        #[weak]
        list_view,
        #[strong]
        history,
        #[strong]
        current_clipboard,
        #[strong]
        display_clone,
        async move {
            let result = gio::spawn_blocking(move || provider.list_entries()).await;

            match result {
                Ok(Ok(entries)) => {
                    eprintln!("cliphist entries loaded: {}", entries.len());
                    for (_raw_id, content) in entries.into_iter().rev() {
                        history
                            .borrow_mut()
                            .add_entry_with_source(content, provider.name().to_string());
                    }

                    let total_entries = history.borrow().entries().len();
                    eprintln!("history entries after import: {}", total_entries);

                    list::refresh_list(
                        &list_view,
                        history.clone(),
                        &display_clone,
                        current_clipboard.clone(),
                    );
                }
                Ok(Err(err)) => {
                    eprintln!("cliphist list error: {err}");
                }
                Err(err) => {
                    eprintln!("cliphist spawn error: {:?}", err);
                }
            }
        }
    ));

    content.append(&list_view);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Clipboard Manager")
        .content(&content)
        .default_height(600)
        .default_width(900)
        .resizable(false)
        .modal(true)
        .build();

    search_bar.set_key_capture_widget(Some(&window));

    let key_controller = gtk::EventControllerKey::new();

    key_controller.connect_key_pressed(glib::clone!(
        #[strong]
        search_button,
        #[strong]
        list_view,
        #[strong]
        current_clipboard,
        #[strong]
        display,
        move |_, key: Key, _key_code, state| {
            if state.contains(gdk::ModifierType::CONTROL_MASK)
                || state.contains(gdk::ModifierType::ALT_MASK)
                || state.contains(gdk::ModifierType::SUPER_MASK)
            {
                return glib::Propagation::Proceed;
            }

            if key == Key::Escape {
                search_button.set_active(false);
                return glib::Propagation::Proceed;
            }

            if key == Key::Down {
                if !list::list_contains_focus(&list_view) && list::select_first_row(&list_view) {
                    return glib::Propagation::Stop;
                }

                if list::move_selection(&list_view, list::NavigationDirection::Down) {
                    return glib::Propagation::Stop;
                }
                return glib::Propagation::Proceed;
            }

            if key == Key::Up {
                if list::move_selection(&list_view, list::NavigationDirection::Up) {
                    return glib::Propagation::Stop;
                }
                return glib::Propagation::Proceed;
            }

            if key == Key::Return || key == Key::KP_Enter {
                if list::activate_selected(&list_view, &history, &display, &current_clipboard) {
                    return glib::Propagation::Stop;
                }
                return glib::Propagation::Proceed;
            }

            if let Some(ch) = key.to_unicode() {
                if ch.is_control() {
                    return glib::Propagation::Proceed;
                }

                if !search_button.is_active() {
                    search_button.set_active(true);
                }
            }
            glib::Propagation::Proceed
        }
    ));

    window.add_controller(key_controller);

    let action_search = gio::SimpleAction::new("search", None);

    action_search.connect_activate(glib::clone!(
        #[weak]
        search_button,
        move |_, _| {
            search_button.set_active(!search_button.is_active());
        }
    ));

    window.add_action(&action_search);

    app.set_accels_for_action("win.search", &["<Control>f"]);

    window.present();
    list::select_first_row(&list_view);
}

fn register_styles(display: &gdk::Display) {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(
        "listboxrow.current-clipboard {\n  background-color: alpha(@accent_bg_color, 0.15);\n}\nlistboxrow.current-clipboard:selected {\n  background-color: alpha(@accent_bg_color, 0.35);\n}\n",
    );
    gtk::style_context_add_provider_for_display(
        display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
