use crate::service::cliboard_history::ClipboardContent;

pub trait IClipboardProvider {
    fn name(&self) -> &'static str;
    fn list_entries(&self) -> Result<Vec<(Option<String>, ClipboardContent)>, String>;
}
