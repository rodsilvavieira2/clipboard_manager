pub mod header;
pub mod list;
pub mod search_bar;
pub mod about;
pub mod shortcuts;

use std::{cell::RefCell, rc::Rc};

use gtk::{
    Orientation,
    gdk::{self},
    gio,
    glib::{self, object::ObjectExt},
    prelude::*,
};
use libadwaita as adw;

use crate::service::{
    cliboard_monitor::{ClipboardMonitor, IClipboardMonitor},
    cliphist_provider::CliphistProvider,
    style_service::StyleService,
};

pub fn build_ui(app: &adw::Application, display: &gdk::Display) {
    let style_service = StyleService::new();
    style_service.apply_styles(display);
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

    let toast_overlay = adw::ToastOverlay::new();
    toast_overlay.set_child(Some(&content));

    let list_view = list::build(
        history.clone(),
        display,
        current_clipboard.clone(),
        toast_overlay.clone(),
    );

    list::setup_search(&list_view, &search_entry);
    list::focus_list(&list_view);

    let provider = CliphistProvider;
    let display_clone = display.clone();
    let toast_overlay_clone = toast_overlay.clone();

    glib::MainContext::default().spawn_local(glib::clone!(
        #[weak]
        list_view,
        #[strong]
        history,
        #[strong]
        current_clipboard,
        #[strong]
        display_clone,
        #[strong]
        toast_overlay_clone,
        #[strong]
        clipboard_monitor,
        async move {
            match clipboard_monitor.load_history(provider).await {
                Ok(_) => {
                    list::refresh_list(
                        &list_view,
                        history,
                        &display_clone,
                        current_clipboard,
                        toast_overlay_clone,
                    );
                }
                Err(err) => eprintln!("Error loading history: {err}"),
            }
        }
    ));

    content.append(&list_view);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Clipboard Manager")
        .content(&toast_overlay)
        .default_height(600)
        .default_width(900)
        .resizable(false)
        .modal(true)
        .build();

    search_bar.set_key_capture_widget(Some(&window));

    let keyboard_service = crate::service::keyboard_service::KeyboardService::new();
    keyboard_service.setup(&window, &search_button, &list_view);

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

    about::setup_about_action(&window);

    shortcuts::setup_shortcuts_action(&window);

    window.present();
    list::select_first_row(&list_view);
}
