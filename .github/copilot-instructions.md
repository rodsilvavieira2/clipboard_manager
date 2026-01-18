# Clipboard Manager AI Instructions

This project is a Linux clipboard manager built with Rust, GTK4, and Libadwaita.

## Tech Stack & Architecture
- **Language**: Rust (Edition 2024)
- **UI Framework**: GTK4 (`gtk4` crate, aliased as `gtk`)
- **Design Language**: Libadwaita (`libadwaita` crate, aliased as `adw`)
- **Entry Point**: `src/main.rs` initializes `adw::Application`.
- **UI Structure**: UI components are modularized under `src/ui/`.
    - `src/ui/mod.rs`: Main UI assembly and window management.
    - `src/ui/header.rs`: Custom header bar components.
    - `src/ui/list.rs`: Clipboard history list implementation using `adw::Clamp` and `gtk::ListBox`.

## Project Conventions

### 1. Naming & Aliases
- Always use `use libadwaita as adw;` for Libadwaita imports.
- Use `gtk` for GTK4 imports (aliased in `Cargo.toml`).

### 2. UI Development Pattern
- Prefer the **Builder Pattern** for widget creation:
  ```rust
  let list_box = gtk::ListBox::builder()
      .selection_mode(gtk::SelectionMode::None)
      .build();
  ```
- Use Libadwaita components for modern GNOME aesthetics:
  - `adw::ApplicationWindow` instead of `gtk::ApplicationWindow`.
  - `adw::HeaderBar` for the top bar.
  - `adw::ActionRow` for list items.
  - `adw::Clamp` to restrict the maximum width of content (usually set to `800`).

### 3. Styling
- Use CSS classes provided by Libadwaita where possible (e.g., `list_box.add_css_class("boxed-list");`).
- Use `flat` class for secondary buttons in rows.

## Critical Workflows
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Linting**: `cargo clippy`

## Integration Points
- Currently, the UI uses placeholder data in `src/ui/list.rs`. When implementing actual clipboard synchronization, look for `gtk::Clipboard` or integration with the Wayland/X11 clipboard via the `arboard` or `gtk::gdk` crates.