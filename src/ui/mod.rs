pub mod header;
pub mod list;

use gtk::{
    Orientation,
    prelude::{BoxExt, GtkWindowExt},
};
use libadwaita as adw;

pub fn build_ui(app: &adw::Application) {
    let header_bar = header::create_header_bar();

    let content = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    content.append(&header_bar);

    let list_view = list::create_list_of_cliboards();
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
