pub mod header;
pub mod list;
pub mod search_bar;

use crate::service::{cliboard_history::IClipboardHistory, cliboard_provider::IClipboardProvider};
use gtk::{
    gdk, gio,
    glib::{self, object::ObjectExt},
    prelude::*,
    Orientation,
};
use libadwaita as adw;

use crate::service::{
    cliboard_monitor::{ClipboardMonitor, IClipboardMonitor},
    cliphist_provider::CliphistProvider,
};

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

    let provider = CliphistProvider;
    let display_clone = display.clone();

    glib::MainContext::default().spawn_local(glib::clone!(
        #[weak]
        list_view,
        #[strong]
        history,
        #[strong]
        display_clone,
        async move {
            let result = gio::spawn_blocking(move || provider.list_entries()).await;

            match result {
                Ok(Ok(entries)) => {
                    eprintln!("cliphist entries loaded: {}", entries.len());
                    for (_raw_id, content) in entries.into_iter().rev() {
                        history.borrow_mut().add_entry_with_source(
                            content,
                            provider.name().to_string(),
                        );
                    }

                    let total_entries = history.borrow().entries().len();
                    eprintln!("history entries after import: {}", total_entries);

                    list::refresh_list(&list_view, history.clone(), &display_clone);
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
