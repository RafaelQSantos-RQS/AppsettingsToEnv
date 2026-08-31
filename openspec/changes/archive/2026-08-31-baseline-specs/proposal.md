## Why

The project has working functionality but no formal specification. To adopt OpenSpec as the source of truth going forward, we need baseline specs that document the current behavior. This establishes the contract for future changes — any modification can be validated against these specs.

## What Changes

- Create initial OpenSpec specs documenting the existing AppsettingsToEnv behavior
- No code changes — this is a documentation-only baseline
- Establishes the spec structure for future development (Flatpak packaging, CLI mode, etc.)

## Capabilities

### New Capabilities

- `conversion/core`: JSON-to-ENV conversion logic — flattening, null handling, escape sequences, Microsoft config-provider convention
- `ui/app`: Desktop UI — two-pane layout, real-time conversion, theme system, about popup
- `integration/clipboard`: Clipboard copy — arboard integration, error handling
- `integration/file-io`: File operations — upload JSON, save as .env, file dialog handling
- `packaging/linux`: Linux packaging — .deb and AppImage builds

### Modified Capabilities

_(none — this is the initial baseline)_

## Impact

- `src/main.rs`: Conversion logic and all integration handlers
- `ui/main.slint`: Full UI definition
- `build.rs`: Slint compilation
- `Cargo.toml`: Dependencies and packaging metadata
- `.github/workflows/`: CI pipeline
