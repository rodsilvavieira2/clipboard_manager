mod service;
mod ui;

use gtk::gio::prelude::{ApplicationExt, ApplicationExtManual};
use libadwaita as adw;

const APP_ID: &str = "org.example.clipmanager";

fn main() {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        let display = gtk::gdk::Display::default().expect("Could not get the default display");

        ui::build_ui(app, &display);
    });

    app.run();
}
