use std::{cell::RefCell, rc::Rc};

use gtk::{gdk, gio};

use crate::service::{
    cliboard_history::{ClipboardHistory, IClipboardHistory},
    cliboard_provider::IClipboardProvider,
};

#[derive(Clone)]
pub struct ClipboardMonitor {
    history: Rc<RefCell<ClipboardHistory>>,
}

pub trait IClipboardMonitor {
    fn new(display: &gdk::Display) -> Self;

    fn history(&self) -> Rc<RefCell<ClipboardHistory>>;
}

impl ClipboardMonitor {
    pub async fn load_history<P: IClipboardProvider + Send + 'static>(
        &self,
        provider: P,
    ) -> Result<(), String> {
        let history = self.history.clone();
        let provider_name = provider.name();
        let result = gio::spawn_blocking(move || provider.list_entries()).await;

        match result {
            Ok(Ok(entries)) => {
                eprintln!("cliphist entries loaded: {}", entries.len());
                let mut history_guard = history.borrow_mut();
                for (_raw_id, content) in entries.into_iter().rev() {
                    history_guard.add_entry_with_source(
                        content,
                        provider_name.to_string(),
                        _raw_id,
                    );
                }
                let total_entries = history_guard.entries().len();
                eprintln!("history entries after import: {}", total_entries);
                Ok(())
            }
            Ok(Err(err)) => Err(format!("Provider error: {err}")),
            Err(err) => Err(format!("Spawn error: {:?}", err)),
        }
    }
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
