use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub enum ClipboardContent {
    Text(String),
    Image(()),
}

impl ClipboardContent {
    pub fn as_text(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Image(_) => "[Image]".to_string(),
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image(_))
    }
}

#[derive(Debug, Clone)]
pub struct ClipboardEntry {
    pub content: ClipboardContent,
    pub timestamp: DateTime<Local>,
    pub source: String,
}

pub trait IClipboardEntry {
    fn new(content: ClipboardContent, source: Option<String>) -> Self;
    fn format_time(&self) -> String;
}

impl IClipboardEntry for ClipboardEntry {
    fn new(content: ClipboardContent, source: Option<String>) -> Self {
        Self {
            content,
            timestamp: Local::now(),
            source: source.unwrap_or_else(|| "Unknown".to_string()),
        }
    }

    fn format_time(&self) -> String {
        let now = Local::now();
        let duration = now.signed_duration_since(self.timestamp);

        if duration.num_seconds() < 60 {
            "Just Now".to_string()
        } else if duration.num_minutes() < 60 {
            format!(
                "{} minute{} ago",
                duration.num_minutes(),
                if duration.num_minutes() == 1 { "" } else { "s" }
            )
        } else if duration.num_hours() < 24 {
            format!(
                "{} hour{} ago",
                duration.num_hours(),
                if duration.num_hours() == 1 { "" } else { "s" }
            )
        } else {
            format!(
                "{} day{} ago",
                duration.num_days(),
                if duration.num_days() == 1 { "" } else { "s" }
            )
        }
    }
}

pub struct ClipboardHistory {
    entries: Vec<ClipboardEntry>,
    max_entries: usize,
}

pub trait IClipboardHistory {
    fn new() -> Self;
    fn add_entry_with_source(&mut self, content: ClipboardContent, source: String);
    fn entries(&self) -> &[ClipboardEntry];
}

impl IClipboardHistory for ClipboardHistory {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 100,
        }
    }

    fn add_entry_with_source(&mut self, content: ClipboardContent, source: String) {
        if let Some(last) = self.entries.first() {
            match (&last.content, &content) {
                (ClipboardContent::Text(a), ClipboardContent::Text(b)) if a == b => return,
                _ => {}
            }
        }

        let entry = ClipboardEntry::new(content, Some(source));
        self.entries.insert(0, entry);

        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }
    }

    fn entries(&self) -> &[ClipboardEntry] {
        &self.entries
    }
}
