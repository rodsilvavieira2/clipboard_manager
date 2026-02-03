use gtk;

pub fn build() -> (gtk::SearchBar, gtk::SearchEntry) {
    let search_enty = gtk::SearchEntry::builder()
        .hexpand(true)
        .placeholder_text("Seach...")
        .build();

    let search_bar = gtk::SearchBar::builder().child(&search_enty).build();

    (search_bar, search_enty)
}
