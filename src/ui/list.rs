use std::{cell::RefCell, rc::Rc};

use gtk::{gdk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use crate::service::cliboard_history::{ClipboardHistory, IClipboardEntry, IClipboardHistory};

pub fn build(history: Rc<RefCell<ClipboardHistory>>, display: &gdk::Display) -> adw::Clamp {
    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(12)
        .margin_end(12)
        .build();

    list_box.add_css_class("boxed-list");

    populate_list(&list_box, &history.borrow(), display);

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

pub fn refresh_list(
    clamp: &adw::Clamp,
    history: Rc<RefCell<ClipboardHistory>>,
    display: &gdk::Display,
) {
    if let Some(scrolled) = clamp.child().and_downcast::<gtk::ScrolledWindow>() {
        if let Some(list_box) = scrolled.child().and_downcast::<gtk::ListBox>() {
            while let Some(child) = list_box.first_child() {
                list_box.remove(&child);
            }

            populate_list(&list_box, &history.borrow(), display);
        }
    }
}

fn populate_list(list_box: &gtk::ListBox, history: &ClipboardHistory, display: &gdk::Display) {
    let entries = history.entries();

    if entries.is_empty() {
        let row = adw::ActionRow::builder()
            .title("No clipboard history yet")
            .subtitle("Copy something to get started")
            .activatable(false)
            .build();
        list_box.append(&row);
        return;
    }

    for entry in entries {
        let content_preview = if entry.content.len() > 100 {
            format!("{}...", &entry.content[..100])
        } else {
            entry.content.clone()
        };

        let row = adw::ActionRow::builder()
            .title(&content_preview)
            .subtitle(&format!("{} • {}", entry.source, entry.format_time()))
            .activatable(true)
            .build();

        let copy_button = gtk::Button::builder()
            .icon_name("edit-copy-symbolic")
            .valign(gtk::Align::Center)
            .css_classes(["flat"])
            .tooltip_text("Copy to clipboard")
            .build();

        let clipboard = display.clipboard();
        let content_to_copy = entry.content.clone();
        copy_button.connect_clicked(move |_| {
            clipboard.set_text(&content_to_copy);
        });

        let clipboard_row = display.clipboard();
        let content_for_row = entry.content.clone();
        row.connect_activated(move |_| {
            clipboard_row.set_text(&content_for_row);
        });

        row.add_suffix(&copy_button);
        list_box.append(&row);
    }
}
