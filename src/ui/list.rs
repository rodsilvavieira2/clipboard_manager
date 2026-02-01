use std::{cell::RefCell, rc::Rc};

use gtk::{gdk, glib, prelude::*};
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
        if let Some(list_box) = find_list_box(&scrolled) {
            while let Some(child) = list_box.first_child() {
                list_box.remove(&child);
            }

            populate_list(&list_box, &history.borrow(), display);
        } else {
            eprintln!("list refresh: list box not found");
        }
    } else {
        eprintln!("list refresh: scrolled window not found");
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
        let content_text = entry.content.as_text();
        let content_preview: String = content_text.chars().take(100).collect();
        let content_preview = if content_preview.len() < content_text.len() {
            format!("{}...", content_preview)
        } else {
            content_preview
        };

        let safe_title = glib::markup_escape_text(&content_preview);
        let safe_subtitle =
            glib::markup_escape_text(&format!("{} • {}", entry.source, entry.format_time()));

        let row = adw::ActionRow::builder()
            .title(safe_title)
            .subtitle(safe_subtitle)
            .activatable(true)
            .build();

        if entry.content.is_image() {
            let icon = gtk::Image::from_icon_name("image-x-generic-symbolic");
            row.add_prefix(&icon);
        }

        let copy_button = gtk::Button::builder()
            .icon_name("edit-copy-symbolic")
            .valign(gtk::Align::Center)
            .css_classes(["flat"])
            .tooltip_text("Copy to clipboard")
            .build();

        let clipboard = display.clipboard();
        let content_to_copy = entry.content.clone();
        copy_button.connect_clicked(move |_| {
            if let crate::service::cliboard_history::ClipboardContent::Text(text) = &content_to_copy
            {
                clipboard.set_text(text);
            }
        });

        let clipboard_row = display.clipboard();
        let content_for_row = entry.content.clone();
        row.connect_activated(move |_| {
            if let crate::service::cliboard_history::ClipboardContent::Text(text) = &content_for_row
            {
                clipboard_row.set_text(text);
            }
        });

        row.add_suffix(&copy_button);
        list_box.append(&row);
    }
}

fn find_list_box(scrolled: &gtk::ScrolledWindow) -> Option<gtk::ListBox> {
    if let Some(list_box) = scrolled.child().and_downcast::<gtk::ListBox>() {
        return Some(list_box);
    }

    let viewport = scrolled.child().and_downcast::<gtk::Viewport>()?;
    viewport.child().and_downcast::<gtk::ListBox>()
}
