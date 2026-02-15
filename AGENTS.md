# AGENTS.md for Clipboard Manager

## Build, Lint, and Test Commands

### Build Commands
- `cargo build` - Builds the project in debug mode.
- `cargo build --release` - Builds the project in release mode optimized for production.
- `cargo build --release --target x86_64-unknown-linux-gnu` - Builds specifically for x86_64 Linux targets.

### Run Commands
- `cargo run` - Build and run the application in debug mode
- `cargo run --release` - Build and run in release mode
- `cargo run -- --help` - Run with help flags

### Test Commands
- `cargo test` - Runs all tests.
- `cargo test <test_function_name>` - Runs a specific test function (e.g., `cargo test test_clipboard_monitor_starts_successfully`).
- `cargo test --lib` - Runs only library tests (excludes integration tests).
- `cargo test --doc` - Runs documentation tests.
- `cargo test -- --nocapture` - Runs tests with output logging enabled for debugging purposes.
- `cargo test -- --ignored` - Runs only tests that were explicitly ignored.
- `cargo test <module_name>::` - Runs all tests in a specific module (e.g., `cargo test clipboard_monitor::`).

### Lint and Format Commands
- `cargo clippy` - Runs the Rust linter to check for common mistakes and style issues.
- `cargo clippy -- -D warnings` - Treats clippy warnings as errors (fail build on warnings).
- `cargo clippy --fix` - Applies auto-fixes suggested by clippy where possible.
- `cargo fmt` - Formats code according to official Rust style guidelines.
- `cargo fmt --check` - Verifies whether the code is correctly formatted without modifying files.
- `cargo check` - Checks code for compilation errors without creating build artifacts.

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

### Naming Conventions
- **Libadwaita**: Always use `use libadwaita as adw;` for imports.
- **GTK4**: Use `gtk` for GTK4 imports (aliased in `Cargo.toml`).
- **Modules**: Use lowercase, `snake_case` (e.g., `clipboard_monitor.rs`, `ui/mod.rs`).
- **Functions**: Use `snake_case` (e.g., `build_ui`, `populate_list`).
- **Variables**: Use `snake_case` (e.g., `clipboard_monitor`, `list_box`).
- **Types**: Use `PascalCase` for structs, enums, and traits (e.g., `ClipboardMonitor`, `IClipboardMonitor`).
- **Constants**: Use `SCREAMING_SNAKE_CASE` (e.g., `APP_ID`).
- **Traits**: Prefix names with 'I' (e.g., `IClipboardMonitor`, `IClipboardHistory`).

### UI Development Guidelines
- **Builder Pattern**: Use the builder pattern for declarative widget creation. Example:
  ```rust
  let list_box = gtk::ListBox::builder()
      .selection_mode(gtk::SelectionMode::None)
      .margin_top(24)
      .margin_bottom(24)
      .margin_start(12)
      .margin_end(12)
      .build();
  ```
- **Libadwaita Components**: Use Libadwaita for GNOME-style modern applications:
  - `adw::ApplicationWindow` instead of `gtk::ApplicationWindow`
  - `adw::HeaderBar` for the top bar
  - `adw::ActionRow` for list items with title, subtitle, and actions
  - `adw::Clamp` for content width restriction (typically `800`)

### Import Organization
- **Grouping**: Organize imports by the following order:
  1. Standard library (`std::`)
  2. Crates (e.g., GTK, GLib, Libadwaita)
  3. Application modules
- **Separation**: Keep GTK/Glib-related imports grouped together.
- **Specific Imports**: Prefer explicit over glob imports to maintain clarity.
- **Example Structure**:
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
- **General Approach**: Use `Result<T, E>` for operations that can fail gracefully.
- **Pattern Matching**: Prefer `if let Ok(value) = ...` over `match` for simple cases.
- **Critical Failures**: Use `expect()` for failures that must panic — include descriptive messages.
- **Async Errors**: Use the `?` operator to propagate errors in asynchronous functions.
- **Example**:
  ```rust
  if let Ok(Some(text)) = clipboard.read_text_future().await {
      // Handle success
  } else {
      // Handle error case
  }
  ```

### Memory Management
- **Shared State**: Use `Rc<RefCell<T>>` for shared mutable state between UI components.
- **GTK Closures**: Use `glib::clone!` macro for closures that capture GTK objects.
- **Weak References**: Use `#[weak]` in `glib::clone!` to avoid reference cycles.
- **Strong References**: Use `#[strong]` for data that needs to outlive the closure.
- **Example**:
  ```rust
  glib::clone!(
      #[weak] widget,
      #[strong] data,
      move || {
          // Closure code: widget is weak, data is strong.
      }
  )
  ```

### Styling
- **CSS Classes**: Use CSS classes provided by Libadwaita where possible.
- **Common Classes**: Utilize classes like `"boxed-list"` and `"flat"` for buttons.
- **Inline Styling**: Avoid inline styling; prefer using predefined CSS classes.
- **Iconography**: Use symbolic icon names (e.g., `"edit-copy-symbolic"`, `"system-search-symbolic"`).

### Code Organization
- **Comments**: Avoid comments unless explaining complex business logic or public API behavior.
- **Traits**: Use traits for interfaces (e.g., `IClipboardMonitor`, `IClipboardHistory`).
- **Function Size**: Keep functions small and focused (under 50 lines where possible).
- **Modules**: Use modules to organize related functionality.
- **Layer Separation**: Separate UI, service, and data handling layers.
- **Grouping**: Group related functionality in dedicated modules to ensure clarity and maintainability.

### Async Programming
- Use `glib::MainContext::default().spawn_local()` for GTK-related async operations
- Prefer async methods over blocking operations in UI code
- Use `glib::clone!` for moving data into async closures
- Handle clipboard operations asynchronously to avoid blocking the UI

### Testing Guidelines
- **Unit Tests**: Write unit tests in the same file as the code they test.
- **Test Modules**: Use `#[cfg(test)]` module for test-specific code.
- **Integration Tests**: Place integration tests in the `tests/` directory (if any).
- **API Testing**: Test public APIs, not internal implementation details.
- **Descriptive Test Names**: Use clear names, e.g., `#[test] fn test_clipboard_monitor_starts_successfully()`.
- **Mocking**: Mock external dependencies when testing service logic.

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