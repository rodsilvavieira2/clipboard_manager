use gtk::{
    gdk::{self, Key},
    glib::{self},
    prelude::*,
};
use libadwaita as adw;

use crate::ui::list;

#[derive(Default, Clone)]
pub struct KeyboardService;

impl KeyboardService {
    pub fn new() -> Self {
        Self
    }

    pub fn setup(
        &self,
        window: &adw::ApplicationWindow,
        search_button: &gtk::ToggleButton,
        list_view: &adw::Clamp,
    ) {
        let key_controller = gtk::EventControllerKey::new();

        key_controller.connect_key_pressed(glib::clone!(
            #[strong]
            search_button,
            #[strong]
            list_view,
            move |_, key, _key_code, state| {
                Self::handle_key_press(&search_button, &list_view, key, state)
            }
        ));

        window.add_controller(key_controller);
    }

    fn handle_key_press(
        search_button: &gtk::ToggleButton,
        list_view: &adw::Clamp,
        key: Key,
        state: gdk::ModifierType,
    ) -> glib::Propagation {
        if state.contains(gdk::ModifierType::CONTROL_MASK)
            || state.contains(gdk::ModifierType::ALT_MASK)
            || state.contains(gdk::ModifierType::SUPER_MASK)
        {
            return glib::Propagation::Proceed;
        }

        if key == Key::Escape {
            search_button.set_active(false);
            return glib::Propagation::Proceed;
        }

        if key == Key::Down {
            if !list::list_contains_focus(list_view) && list::select_first_row(list_view) {
                return glib::Propagation::Stop;
            }

            if list::move_selection(list_view, list::NavigationDirection::Down) {
                return glib::Propagation::Stop;
            }
            return glib::Propagation::Proceed;
        }

        if key == Key::Up {
            if list::move_selection(list_view, list::NavigationDirection::Up) {
                return glib::Propagation::Stop;
            }
            return glib::Propagation::Proceed;
        }

        if let Some(ch) = key.to_unicode() {
            if ch.is_control() {
                return glib::Propagation::Proceed;
            }

            if !search_button.is_active() {
                search_button.set_active(true);
            }
        }
        glib::Propagation::Proceed
    }
}
