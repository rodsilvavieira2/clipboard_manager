use gtk::prelude::*;
use libadwaita::{self as adw, prelude::*};

pub fn build() -> adw::Clamp {
    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(12)
        .margin_end(12)
        .build();

    list_box.add_css_class("boxed-list");

    for i in 1..=10 {
        let row = adw::ActionRow::builder()
            .title(format!("Example clipboard content #{}", i))
            .subtitle("Copied from terminal • 2 minutes ago")
            .activatable(true)
            .build();

        let copy_button = gtk::Button::builder()
            .icon_name("edit-copy-symbolic")
            .valign(gtk::Align::Center)
            .css_classes(["flat"])
            .build();

        row.add_suffix(&copy_button);
        list_box.append(&row);
    }

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&list_box)
        .vexpand(true)
        .build();

    adw::Clamp::builder()
        .maximum_size(800)
        .child(&scrolled_window)
        .build()
}
