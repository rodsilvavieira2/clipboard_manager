use std::{cell::RefCell, rc::Rc};

use gtk::gdk::{self, glib, prelude::DisplayExt};

use crate::service::cliboard_history::{ClipboardContent, ClipboardHistory, IClipboardHistory};

pub struct ClipboardMonitor {
    clipboard: gdk::Clipboard,
    history: Rc<RefCell<ClipboardHistory>>,
}

pub trait IClipboardMonitor {
    fn new(display: &gdk::Display) -> Self;

    fn history(&self) -> Rc<RefCell<ClipboardHistory>>;

    fn check_clipboard<F>(&self, on_change: F)
    where
        F: Fn() + 'static;

    fn start_monitoring<F>(&self, on_change: F)
    where
        F: Fn() + 'static + Clone;
}

impl IClipboardMonitor for ClipboardMonitor {
    fn new(display: &gdk::Display) -> Self {
        let clipboard = display.clipboard();
        let history = Rc::new(RefCell::new(ClipboardHistory::new()));

        Self { clipboard, history }
    }

    fn history(&self) -> Rc<RefCell<ClipboardHistory>> {
        Rc::clone(&self.history)
    }

    fn check_clipboard<F>(&self, on_change: F)
    where
        F: Fn() + 'static,
    {
        let clipboard = &self.clipboard;
        let history = &self.history;

        glib::MainContext::default().spawn_local(glib::clone!(
            #[weak]
            clipboard,
            #[strong]
            history,
            async move {
                if let Ok(text) = clipboard.read_text_future().await {
                    if let Some(text) = text {
                        let text_str = text.to_string();

                        if !text_str.is_empty() {
                            history
                                .borrow_mut()
                                .add_entry(ClipboardContent::Text(text_str));
                            on_change()
                        }
                    }
                }
            }
        ));
    }

    fn start_monitoring<F>(&self, on_change: F)
    where
        F: Fn() + 'static + Clone,
    {
        let clipboard = &self.clipboard;
        let history = &self.history;

        self.clipboard.connect_changed(glib::clone!(
            #[weak]
            clipboard,
            #[strong]
            history,
            move |_| {
                let callback = on_change.clone();

                glib::MainContext::default().spawn_local(glib::clone!(
                    #[weak]
                    clipboard,
                    #[strong]
                    history,
                    async move {
                        if let Ok(Some(text)) = clipboard.read_text_future().await {
                            let text_str = text.to_string();
                            if !text_str.is_empty() {
                                history
                                    .borrow_mut()
                                    .add_entry(ClipboardContent::Text(text_str));
                                callback();
                            }
                        }
                    }
                ));
            }
        ));
    }
}
