use gtk;
use libadwaita as adw;

pub fn build() -> (adw::HeaderBar, gtk::ToggleButton) {
    let search_button = gtk::ToggleButton::builder()
        .icon_name("system-search-symbolic")
        .tooltip_text("Search clipboard history")
        .build();

    let header = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::new(Some("Clipboard Manager")))
        .show_title(true)
        .build();

    header.pack_start(&search_button);

    (header, search_button)
}
