use std::{cell::RefCell, rc::Rc};

use gtk::gdk;

use crate::service::cliboard_history::{ClipboardHistory, IClipboardHistory};

pub struct ClipboardMonitor {
    history: Rc<RefCell<ClipboardHistory>>,
}

pub trait IClipboardMonitor {
    fn new(display: &gdk::Display) -> Self;

    fn history(&self) -> Rc<RefCell<ClipboardHistory>>;
}

impl IClipboardMonitor for ClipboardMonitor {
    fn new(_display: &gdk::Display) -> Self {
        let history = Rc::new(RefCell::new(ClipboardHistory::new()));

        Self { history }
    }

    fn history(&self) -> Rc<RefCell<ClipboardHistory>> {
        Rc::clone(&self.history)
    }
}
