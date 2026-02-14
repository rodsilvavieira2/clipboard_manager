use gtk::gdk;

pub struct StyleService;

impl StyleService {
    pub fn new() -> Self {
        Self
    }

    pub fn apply_styles(&self, display: &gdk::Display) {
        let provider = gtk::CssProvider::new();
        provider.load_from_string(
            "listboxrow.current-clipboard {\n  background-color: alpha(@accent_bg_color, 0.15);\n}\nlistboxrow.current-clipboard:selected {\n  background-color: alpha(@accent_bg_color, 0.35);\n}\n",
        );
        gtk::style_context_add_provider_for_display(
            display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

impl Default for StyleService {
    fn default() -> Self {
        Self::new()
    }
}
