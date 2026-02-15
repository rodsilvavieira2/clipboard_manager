use gtk;
use gtk::gio;
use libadwaita as adw;

pub fn build() -> (adw::HeaderBar, gtk::ToggleButton) {
    let search_button = gtk::ToggleButton::builder()
        .icon_name("system-search-symbolic")
        .tooltip_text("Search clipboard history")
        .build();

    let menu = gio::Menu::new();
    menu.append(Some("Shortcuts"), Some("win.show-shortcuts"));
    menu.append(Some("About"), Some("win.show-about"));

    let menu_button = gtk::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&menu)
        .tooltip_text("Main Menu")
        .build();

    let header = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::new(Some("Clipboard Manager")))
        .show_title(true)
        .build();

    header.pack_start(&search_button);
    header.pack_end(&menu_button);

    (header, search_button)
}
