use gtk::{gio, glib, prelude::*};
use libadwaita as adw;

pub fn setup_about_action(window: &adw::ApplicationWindow) {
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
}
