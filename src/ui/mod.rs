pub mod header;
pub mod list;
pub mod search_bar;

use std::{cell::RefCell, rc::Rc};

use crate::service::{cliboard_history::IClipboardHistory, cliboard_provider::IClipboardProvider};
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
        async move {
            let result = gio::spawn_blocking(move || provider.list_entries()).await;

            match result {
                Ok(Ok(entries)) => {
                    eprintln!("cliphist entries loaded: {}", entries.len());
                    for (_raw_id, content) in entries.into_iter().rev() {
                        history.borrow_mut().add_entry_with_source(
                            content,
                            provider.name().to_string(),
                            _raw_id,
                        );
                    }

                    let total_entries = history.borrow().entries().len();
                    eprintln!("history entries after import: {}", total_entries);

                    list::refresh_list(
                        &list_view,
                        history.clone(),
                        &display_clone,
                        current_clipboard.clone(),
                        toast_overlay_clone,
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

    let action_about = gio::SimpleAction::new("show-about", None);
    action_about.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_, _| {
            let about = adw::AboutWindow::builder()
                .application_name("Clipboard Manager")
                .developer_name("Rodrigo")
                .version("0.1.0")
                .comments("A simple clipboard manager built with Rust and GTK4.")
                .website("https://github.com/rodsilvavieira2/clipboard_manager")
                .issue_url("https://github.com/rodsilvavieira2/clipboard_manager/issues")
                .license_type(gtk::License::MitX11)
                .modal(true)
                .transient_for(&window)
                .build();
            about.present();
        }
    ));
    window.add_action(&action_about);

    let action_shortcuts = gio::SimpleAction::new("show-shortcuts", None);
    action_shortcuts.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_, _| {
            let shortcuts = gtk::ShortcutsWindow::builder()
                .modal(true)
                .transient_for(&window)
                .build();

            let section = gtk::ShortcutsSection::builder()
                .section_name("main")
                .title("General")
                .build();

            let group = gtk::ShortcutsGroup::builder().title("Application").build();

            let shortcut_search = gtk::ShortcutsShortcut::builder()
                .title("Search")
                .accelerator("<Control>f")
                .build();

            let shortcut_quit = gtk::ShortcutsShortcut::builder()
                .title("Quit")
                .accelerator("<Control>q")
                .build();

            group.append(&shortcut_search);
            group.append(&shortcut_quit);
            section.append(&group);
            shortcuts.set_child(Some(&section));

            shortcuts.present();
        }
    ));
    window.add_action(&action_shortcuts);

    window.present();
    list::select_first_row(&list_view);
}
