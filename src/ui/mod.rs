pub mod header;
pub mod list;
pub mod search_bar;

use gtk::{
    Orientation,
    glib::object::ObjectExt,
    prelude::{BoxExt, GtkWindowExt},
};
use libadwaita as adw;

pub fn build_ui(app: &adw::Application) {
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

    let list_view = list::build();

    content.append(&list_view);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Clipboard Manager")
        .content(&content)
        .default_height(600)
        .default_width(900)
        .build();

    window.present();
}
