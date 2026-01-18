pub mod header;
pub mod list;
pub mod search_bar;

use gtk::{
    Orientation, gdk, gio,
    glib::{self, object::ObjectExt},
    prelude::*,
};
use libadwaita as adw;

use crate::service::cliboard_monitor::{ClipboardMonitor, IClipboardMonitor};

pub fn build_ui(app: &adw::Application, display: &gdk::Display) {
    let (header_bar, search_button) = header::build();

    let content = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    content.append(&header_bar);

    let search_bar = search_bar::build();

    search_button
        .bind_property("active", &search_bar, "search-mode-enabled")
        .sync_create()
        .bidirectional()
        .build();

    content.append(&search_bar);

    let clipboard_monitor = ClipboardMonitor::new(display);
    let history = clipboard_monitor.history();

    let list_view = list::build(history.clone(), display);

    let display_clone = display.clone();

    clipboard_monitor.start_monitoring(glib::clone!(
        #[weak]
        list_view,
        #[strong]
        history,
        #[strong]
        display_clone,
        move || {
            list::refresh_list(&list_view, history.clone(), &display_clone);
        }
    ));

    content.append(&list_view);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Clipboard Manager")
        .content(&content)
        .default_height(600)
        .default_width(900)
        .build();

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
}
