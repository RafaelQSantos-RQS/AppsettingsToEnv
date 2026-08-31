# Contributing

Thanks for wanting to contribute! This project is small on purpose — please keep changes minimal and focused.

## Development environment

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- System libraries for Slint: `libx11-dev`, `libxkbcommon-dev`, `libwayland-dev`, `libgl-dev`
- `pkg-config`

On Debian/Ubuntu:

```bash
sudo apt install pkg-config libx11-dev libxkbcommon-dev libwayland-dev libgl-dev
```

### Getting started

```bash
cargo run    # run the app
cargo test   # run the tests
```

Before opening a PR, make sure everything is clean:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

The CI runs exactly these three checks, so a PR that doesn't pass them won't be merged.

## IDE setup

[VS Code](https://code.visualstudio.com/) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension is recommended. It provides inline errors, auto-imports, and type hints.

## Conventions

- Keep the diff small. One feature or fix per PR.
- UI text stays in English (the whole app is in English).
- Commit messages follow the repo's existing style: short, in English (see `git log`).
- New SVG icons follow the existing style (lucide-style, matching the palette).

## Reporting issues

- Bugs: use the **Bug report** template.
- Ideas: use the **Feature request** template.

That's it. Nothing else is required.
