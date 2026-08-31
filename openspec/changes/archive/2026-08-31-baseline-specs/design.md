## Context

Single-binary Rust desktop app. UI is Slint (compiled at build time via `slint-build`), business logic is one file (`src/main.rs`). No network, no CLI, no plugins. See proposal.md for motivation.

## Goals / Non-Goals

**Goals:**
- Document the current architecture as-is
- Establish the baseline for future spec-driven development

**Non-Goals:**
- Refactoring or restructuring
- Adding new features
- Performance optimization

## Decisions

### Single-file architecture
All Rust logic lives in `src/main.rs`: the `flatten` function, `convert` function, UI callback wiring, and clipboard/file-dialog handlers.

**Why**: The codebase is ~190 lines. Splitting into modules would add complexity without benefit. The conversion logic is a pure function; the UI handlers are thin wrappers around library calls.

**Alternative considered**: Separate `conversion.rs`, `ui.rs`, `clipboard.rs` modules. Rejected — YAGNI for this size.

### Slint for UI
Slint compiles `.slint` files at build time into Rust types (`MainWindow::new()`). The UI is declarative with global theme state and callback-based data flow.

**Why**: Single-binary, no web runtime, native look. The `fluent` style matches Windows/Linux desktop feel.

**Alternative considered**: GTK (heavier dependency), egui (immediate mode, less declarative). Slint fits the fixed-layout, callback-driven pattern.

### Recursive flatten with prefix accumulation
`flatten(value, prefix, out)` recurses through objects/arrays, accumulating `__`-separated keys. Null is skipped at the match arm; scalars are formatted inline.

**Why**: Handles arbitrary nesting depth without explicit stack. The prefix-based approach naturally produces the `Key__SubKey__0` format Microsoft expects.

### arboard for clipboard
Persistent `arboard::Clipboard` instance (not created per-copy) — a dropped `Clipboard` is invisible to X11 window managers.

**Why**: X11 clipboard ownership is tied to the `Clipboard` object lifetime. Creating and dropping it per-copy loses the data.

### File dialogs on worker threads
`rfd::FileDialog` blocks its thread. Both upload and save handlers spawn `std::thread::spawn` and use `slint::invoke_from_event_loop` to push results back to the UI thread.

**Why**: Slint's event loop is single-threaded. Blocking it freezes the UI.

## Risks / Trade-offs

- **Single-file growth** → Acceptable at current size (~190 lines). If the app grows past ~500 lines, split into modules.
- **Fixed window size** → Intentional for v1. Resizing support would require layout rework in Slint.
- **No drag-and-drop** → File dialog is the only input method. Drag-and-drop would require Slint platform integration.
