# AGENTS.md for Clipboard Manager

## Build, Lint, and Test Commands

### Build Commands
- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build the project in release mode with optimizations
- `cargo build --release --target x86_64-unknown-linux-gnu` - Build for specific Linux target

### Run Commands
- `cargo run` - Build and run the application in debug mode
- `cargo run --release` - Build and run in release mode
- `cargo run -- --help` - Run with help flags

### Test Commands
- `cargo test` - Run all tests
- `cargo test <test_function_name>` - Run a specific test function (e.g., `cargo test test_clipboard_monitor_starts_successfully`)
- `cargo test --lib` - Run only library tests (not integration tests)
- `cargo test --doc` - Run documentation tests
- `cargo test -- --nocapture` - Run tests with output capture disabled for debugging
- `cargo test -- --ignored` - Run only ignored tests
- `cargo test <module_name>::` - Run tests for a specific module (e.g., `cargo test cliboard_monitor::`)

### Lint and Format Commands
- `cargo clippy` - Run the Rust linter to check for common mistakes and style issues
- `cargo clippy -- -D warnings` - Treat clippy warnings as errors
- `cargo clippy --fix` - Automatically fix clippy suggestions where possible
- `cargo fmt` - Format code according to Rust standards
- `cargo fmt --check` - Check if code is properly formatted without making changes
- `cargo check` - Check code for compilation errors without building

### Development Workflow
- `cargo watch -x run` - Auto-restart on file changes (requires cargo-watch)
- `cargo flamegraph --bin clipboard_manager` - Profile performance (requires cargo-flamegraph)

## Code Style Guidelines

### Project Overview
This is a Linux clipboard manager built with Rust, GTK4, and Libadwaita. The application monitors system clipboard changes and maintains a history of copied items with a modern GNOME-style interface.

### Tech Stack & Architecture
- **Language**: Rust (Edition 2024)
- **UI Framework**: GTK4 (`gtk4` crate, aliased as `gtk`)
- **Design Language**: Libadwaita (`libadwaita` crate, aliased as `adw`)
- **Entry Point**: `src/main.rs` initializes `adw::Application`
- **UI Structure**: UI components are modularized under `src/ui/`
- **Service Layer**: Clipboard monitoring and history management in `src/service/`
- **Dependencies**: chrono for timestamps, gtk4 and libadwaita for UI

### Naming & Aliases
- Always use `use libadwaita as adw;` for Libadwaita imports
- Use `gtk` for GTK4 imports (aliased in `Cargo.toml`)
- Module names: lowercase, snake_case (e.g., `clipboard_monitor.rs`, `ui/mod.rs`)
- Function names: snake_case (e.g., `build_ui`, `populate_list`)
- Variable names: snake_case (e.g., `clipboard_monitor`, `list_box`)
- Type names: PascalCase (structs, enums, traits) (e.g., `ClipboardMonitor`, `IClipboardMonitor`)
- Constants: SCREAMING_SNAKE_CASE (e.g., `APP_ID`)
- Trait names: PascalCase with 'I' prefix for interfaces (e.g., `IClipboardMonitor`)

### UI Development Pattern
- Prefer the **Builder Pattern** for widget creation:
  ```rust
  let list_box = gtk::ListBox::builder()
      .selection_mode(gtk::SelectionMode::None)
      .margin_top(24)
      .margin_bottom(24)
      .margin_start(12)
      .margin_end(12)
      .build();
  ```
- Use Libadwaita components for modern GNOME aesthetics:
  - `adw::ApplicationWindow` instead of `gtk::ApplicationWindow`
  - `adw::HeaderBar` for the top bar
  - `adw::ActionRow` for list items with title, subtitle, and actions
  - `adw::Clamp` to restrict the maximum width of content (usually set to `800`)

### Import Organization
- Group imports by crate, then standard library
- Separate GTK/Glib related imports
- Use explicit imports rather than glob imports when possible
- Example structure:
  ```rust
  use std::{cell::RefCell, rc::Rc};

  use gtk::{
      gdk, gio,
      glib::{self, object::ObjectExt},
      prelude::*,
  };
  use libadwaita as adw;

  use crate::service::clipboard_monitor::{ClipboardMonitor, IClipboardMonitor};
  ```

### Error Handling
- Use `Result<T, E>` for operations that can fail
- Prefer `if let Ok(value) = ...` over `match` for simple cases
- Use `expect()` for critical failures that should panic (with descriptive messages)
- Handle async errors with `?` operator in async functions
- Example:
  ```rust
  if let Ok(Some(text)) = clipboard.read_text_future().await {
      // handle success
  } else {
      // handle error case
  }
  ```

### Memory Management
- Use `Rc<RefCell<T>>` for shared mutable state between UI components
- Use `glib::clone!` macro when creating closures that capture GTK objects
- Use weak references (`#[weak]`) in glib::clone! to avoid reference cycles
- Strong references (`#[strong]`) for data that needs to outlive the closure
- Example:
  ```rust
  glib::clone!(
      #[weak] widget,
      #[strong] data,
      move || {
          // closure code - widget is weak, data is strong
      }
  )
  ```

### Styling
- Use CSS classes provided by Libadwaita where possible
- Common classes: `"boxed-list"`, `"flat"` for buttons
- Avoid inline styling; prefer CSS classes
- Use symbolic icon names (e.g., `"edit-copy-symbolic"`, `"system-search-symbolic"`)

### Code Organization
- No comments unless explaining complex business logic or public API behavior
- Use traits for interfaces (e.g., `IClipboardMonitor`, `IClipboardHistory`)
- Keep functions small and focused (under 50 lines when possible)
- Use modules to organize related functionality
- Separate UI, service, and data layers
- Group related functionality in dedicated modules

### Async Programming
- Use `glib::MainContext::default().spawn_local()` for GTK-related async operations
- Prefer async methods over blocking operations in UI code
- Use `glib::clone!` for moving data into async closures
- Handle clipboard operations asynchronously to avoid blocking the UI

### Testing Guidelines
- Write unit tests in the same file as the code they test
- Use `#[cfg(test)]` module for test-specific code
- Integration tests go in `tests/` directory (if any)
- Test public APIs, not implementation details
- Use descriptive test names: `#[test] fn test_clipboard_monitor_starts_successfully()`
- Mock external dependencies when testing service logic

### Security Considerations
- Never log or expose sensitive clipboard content
- Validate clipboard content before processing (check for emptiness)
- Use secure clipboard APIs provided by GTK
- Handle errors gracefully without exposing internal state
- Clear clipboard history when appropriate

### Dependencies
- Check `Cargo.toml` before adding new dependencies
- Prefer well-maintained crates from the Rust ecosystem
- Use specific version numbers, not wildcards
- Review crate documentation and security advisories
- Only add dependencies when necessary (prefer std library solutions)

### Performance Considerations
- GTK applications run on the main thread; avoid blocking operations
- Use async operations for I/O and network calls (though not applicable here)
- Minimize UI updates; batch changes when possible
- Profile with `cargo flamegraph` if performance issues arise
- Limit clipboard history size to prevent memory bloat (currently 100 entries)

## Copilot Instructions Integration

### Project Conventions (from .github/copilot-instructions.md)

#### 1. Naming & Aliases
- Always use `use libadwaita as adw;` for Libadwaita imports.
- Use `gtk` for GTK4 imports (aliased in `Cargo.toml`).

#### 2. UI Development Pattern
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

#### 3. Styling
- Use CSS classes provided by Libadwaita where possible (e.g., `list_box.add_css_class("boxed-list");`).
- Use `flat` class for secondary buttons in rows.

### Critical Workflows
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Linting**: `cargo clippy`

### Integration Points
- Currently, the UI uses placeholder data in `src/ui/list.rs`. When implementing actual clipboard synchronization, look for `gtk::Clipboard` or integration with the Wayland/X11 clipboard via the `arboard` or `gtk::gdk` crates.

## Additional Development Notes

### File Structure
```
src/
├── main.rs                 # Application entry point
├── ui/                     # User interface components
│   ├── mod.rs             # UI module assembly
│   ├── header.rs          # Header bar implementation
│   ├── list.rs            # Clipboard history list
│   └── search_bar.rs      # Search functionality
└── service/               # Business logic
    ├── mod.rs            # Service module exports
    ├── clipboard_monitor.rs  # Clipboard monitoring
    └── clipboard_history.rs  # History management
```

### Key Implementation Details
- Clipboard monitoring uses GTK's `gdk::Clipboard` with change signals
- History is stored in memory with a maximum of 100 entries
- Time formatting uses chrono crate with relative timestamps
- UI updates happen asynchronously to prevent blocking
- Widgets are built using the builder pattern for declarative construction

### Future Enhancements
- Consider adding persistence for clipboard history
- Implement search/filter functionality in the UI
- Add keyboard shortcuts for navigation
- Support for different clipboard content types (images, etc.)
- Settings for history size and retention policies