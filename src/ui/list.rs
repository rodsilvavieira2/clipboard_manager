use std::{cell::RefCell, rc::Rc};

use gtk::{gdk, glib, prelude::*};
use libadwaita::{self as adw, prelude::*};

use crate::service::cliboard_history::{ClipboardHistory, IClipboardEntry, IClipboardHistory};

const CURRENT_CLIPBOARD_CLASS: &str = "current-clipboard";

pub fn build(
    history: Rc<RefCell<ClipboardHistory>>,
    display: &gdk::Display,
    current_clipboard: Rc<RefCell<Option<String>>>,
) -> adw::Clamp {
    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::Single)
        .can_focus(true)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(12)
        .margin_end(12)
        .build();

    list_box.add_css_class("boxed-list");

    let display_clone = display.clone();
    let history_clone = history.clone();
    let current_clipboard_clone = current_clipboard.clone();

    list_box.connect_row_activated(move |list_box, row| {
        let index = row.index();

        if index < 0 {
            return;
        }

        let entry = {
            let history = history_clone.borrow();
            let Some(entry) = history.entries().get(index as usize) else {
                return;
            };
            entry.clone()
        };

        if let crate::service::cliboard_history::ClipboardContent::Text(text) = entry.content {
            display_clone.clipboard().set_text(&text);
            set_current_clipboard(list_box, &history_clone, &current_clipboard_clone, &text);
        }
    });

    populate_list(&list_box, history, display, current_clipboard);

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
    current_clipboard: Rc<RefCell<Option<String>>>,
) {
    if let Some(scrolled) = clamp.child().and_downcast::<gtk::ScrolledWindow>() {
        if let Some(list_box) = find_list_box(&scrolled) {
            while let Some(child) = list_box.first_child() {
                list_box.remove(&child);
            }

            populate_list(&list_box, history, display, current_clipboard);
            select_first_row(clamp);
        } else {
            eprintln!("list refresh: list box not found");
        }
    } else {
        eprintln!("list refresh: scrolled window not found");
    }
}

fn populate_list(
    list_box: &gtk::ListBox,
    history: Rc<RefCell<ClipboardHistory>>,
    display: &gdk::Display,
    current_clipboard: Rc<RefCell<Option<String>>>,
) {
    let entries = history.borrow().entries().to_vec();

    if entries.is_empty() {
        let row = adw::ActionRow::builder()
            .title("No clipboard history yet")
            .subtitle("Copy something to get started")
            .activatable(false)
            .build();
        let list_row = gtk::ListBoxRow::new();
        list_row.set_child(Some(&row));
        list_box.append(&list_row);
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

        let list_row = gtk::ListBoxRow::new();
        list_row.set_child(Some(&row));

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
        let current_clipboard_for_button = current_clipboard.clone();
        let history_for_button = history.clone();
        copy_button.connect_clicked(glib::clone!(
            #[weak]
            list_box,
            move |_| {
                if let crate::service::cliboard_history::ClipboardContent::Text(text) =
                    &content_to_copy
                {
                    clipboard.set_text(text);
                    set_current_clipboard(
                        &list_box,
                        &history_for_button,
                        &current_clipboard_for_button,
                        text,
                    );
                }
            }
        ));

        row.add_suffix(&copy_button);
        list_box.append(&list_row);
    }

    let history_snapshot = history.borrow();
    apply_current_highlight(
        list_box,
        &history_snapshot,
        current_clipboard.borrow().as_deref(),
    );
}

fn find_list_box(scrolled: &gtk::ScrolledWindow) -> Option<gtk::ListBox> {
    if let Some(list_box) = scrolled.child().and_downcast::<gtk::ListBox>() {
        return Some(list_box);
    }

    let viewport = scrolled.child().and_downcast::<gtk::Viewport>()?;
    viewport.child().and_downcast::<gtk::ListBox>()
}

pub fn setup_search(clamp: &adw::Clamp, search_entry: &gtk::SearchEntry) {
    if let Some(scrolled) = clamp.child().and_downcast::<gtk::ScrolledWindow>()
        && let Some(list_box) = find_list_box(&scrolled)
    {
        list_box.set_filter_func(glib::clone!(
            #[strong]
            search_entry,
            move |row| {
                let text = search_entry.text();

                if text.is_empty() {
                    return true;
                }

                if let Some(action_row) = row.child().and_downcast::<adw::ActionRow>() {
                    let title = action_row.title();

                    return title.to_lowercase().contains(&text.to_lowercase());
                }

                true
            }
        ));

        search_entry.connect_search_changed(glib::clone!(
            #[weak]
            list_box,
            move |_| {
                list_box.invalidate_filter();
                ensure_visible_selection(&list_box);
            }
        ));
    }
}

pub fn focus_list(clamp: &adw::Clamp) {
    if let Some(scrolled) = clamp.child().and_downcast::<gtk::ScrolledWindow>()
        && let Some(list_box) = find_list_box(&scrolled)
    {
        list_box.grab_focus();

        if list_box.selected_row().is_none()
            && !is_placeholder_row(&list_box)
            && let Some(row) = first_visible_row(&list_box)
        {
            list_box.select_row(Some(&row));
        }
    }
}

#[derive(Clone, Copy)]
pub enum NavigationDirection {
    Up,
    Down,
}

pub fn move_selection(clamp: &adw::Clamp, direction: NavigationDirection) -> bool {
    let Some(scrolled) = clamp.child().and_downcast::<gtk::ScrolledWindow>() else {
        return false;
    };
    let Some(list_box) = find_list_box(&scrolled) else {
        return false;
    };

    if is_placeholder_row(&list_box) {
        return false;
    }

    list_box.grab_focus();

    let selected = list_box.selected_row().filter(|row| row.is_visible());
    let next_row = match (direction, selected.as_ref()) {
        (NavigationDirection::Down, Some(row)) => next_visible_row(row),
        (NavigationDirection::Up, Some(row)) => prev_visible_row(row),
        (NavigationDirection::Down, None) => first_visible_row(&list_box),
        (NavigationDirection::Up, None) => last_visible_row(&list_box),
    };

    if let Some(row) = next_row {
        list_box.select_row(Some(&row));
        return true;
    }

    false
}

pub fn select_first_row(clamp: &adw::Clamp) -> bool {
    let Some(scrolled) = clamp.child().and_downcast::<gtk::ScrolledWindow>() else {
        return false;
    };
    let Some(list_box) = find_list_box(&scrolled) else {
        return false;
    };

    if is_placeholder_row(&list_box) {
        return false;
    }

    if let Some(row) = first_visible_row(&list_box) {
        list_box.select_row(Some(&row));
        list_box.grab_focus();
        return true;
    }

    false
}

pub fn list_contains_focus(clamp: &adw::Clamp) -> bool {
    if let Some(scrolled) = clamp.child().and_downcast::<gtk::ScrolledWindow>()
        && let Some(list_box) = find_list_box(&scrolled)
    {
        return list_box.has_focus() || list_box.focus_child().is_some();
    }

    false
}

fn is_placeholder_row(list_box: &gtk::ListBox) -> bool {
    let Some(first_child) = list_box.first_child().and_downcast::<gtk::ListBoxRow>() else {
        return false;
    };

    if first_child.next_sibling().is_some() {
        return false;
    }

    let Some(row) = first_child.child().and_downcast::<adw::ActionRow>() else {
        return false;
    };

    row.title() == "No clipboard history yet"
}

fn set_current_clipboard(
    list_box: &gtk::ListBox,
    history: &Rc<RefCell<ClipboardHistory>>,
    current_clipboard: &Rc<RefCell<Option<String>>>,
    text: &str,
) {
    *current_clipboard.borrow_mut() = Some(text.to_string());
    let history_snapshot = history.borrow();
    apply_current_highlight(list_box, &history_snapshot, Some(text));
}

fn apply_current_highlight(
    list_box: &gtk::ListBox,
    history: &ClipboardHistory,
    current_text: Option<&str>,
) {
    let entries = history.entries();
    let mut row = list_box.first_child().and_downcast::<gtk::ListBoxRow>();
    let mut matched = false;

    while let Some(current) = row {
        let row_text = usize::try_from(current.index())
            .ok()
            .and_then(|index| entries.get(index))
            .and_then(|entry| match &entry.content {
                crate::service::cliboard_history::ClipboardContent::Text(text) => Some(text),
                crate::service::cliboard_history::ClipboardContent::Image(_) => None,
            });
        let is_match = !matched
            && current_text.is_some()
            && row_text.is_some_and(|value| current_text == Some(value.as_str()));

        if is_match {
            current.add_css_class(CURRENT_CLIPBOARD_CLASS);
            matched = true;
        } else {
            current.remove_css_class(CURRENT_CLIPBOARD_CLASS);
        }

        row = current.next_sibling().and_downcast::<gtk::ListBoxRow>();
    }
}

fn first_visible_row(list_box: &gtk::ListBox) -> Option<gtk::ListBoxRow> {
    let mut row = list_box.first_child().and_downcast::<gtk::ListBoxRow>();

    while let Some(current) = row {
        if current.is_visible() {
            return Some(current);
        }

        row = current.next_sibling().and_downcast::<gtk::ListBoxRow>();
    }

    None
}

fn last_visible_row(list_box: &gtk::ListBox) -> Option<gtk::ListBoxRow> {
    let mut row = list_box.last_child().and_downcast::<gtk::ListBoxRow>();

    while let Some(current) = row {
        if current.is_visible() {
            return Some(current);
        }

        row = current.prev_sibling().and_downcast::<gtk::ListBoxRow>();
    }

    None
}

fn next_visible_row(row: &gtk::ListBoxRow) -> Option<gtk::ListBoxRow> {
    let mut next = row.next_sibling().and_downcast::<gtk::ListBoxRow>();

    while let Some(candidate) = next {
        if candidate.is_visible() {
            return Some(candidate);
        }

        next = candidate.next_sibling().and_downcast::<gtk::ListBoxRow>();
    }

    None
}

fn prev_visible_row(row: &gtk::ListBoxRow) -> Option<gtk::ListBoxRow> {
    let mut prev = row.prev_sibling().and_downcast::<gtk::ListBoxRow>();

    while let Some(candidate) = prev {
        if candidate.is_visible() {
            return Some(candidate);
        }

        prev = candidate.prev_sibling().and_downcast::<gtk::ListBoxRow>();
    }

    None
}

fn ensure_visible_selection(list_box: &gtk::ListBox) {
    match list_box.selected_row() {
        Some(selected) => {
            if selected.is_visible() {
                return;
            }
        }
        None => {
            return;
        }
    }

    if let Some(row) = first_visible_row(list_box) {
        list_box.select_row(Some(&row));
    } else {
        list_box.select_row(None::<&gtk::ListBoxRow>);
    }
}
