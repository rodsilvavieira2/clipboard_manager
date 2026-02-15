use gtk::{gio, glib, prelude::*};
use libadwaita as adw;

pub fn setup_shortcuts_action(window: &adw::ApplicationWindow) {
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
}
