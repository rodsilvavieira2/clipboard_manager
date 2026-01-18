use gtk;

pub fn build() -> gtk::SearchBar {
    let search_enty = gtk::SearchEntry::builder()
        .hexpand(true)
        .placeholder_text("Seach...")
        .build();

    gtk::SearchBar::builder().child(&search_enty).build()
}
