use gtk;
use libadwaita as adw;

pub fn create_header_bar() -> adw::HeaderBar {
    let header = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::new(Some("Clipboard Manager")))
        .show_title(true)
        .build();

    header
}
