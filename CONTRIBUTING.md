# Contributing

Thanks for wanting to contribute! This project is small on purpose — please keep changes minimal and focused.

## Getting started

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

## Conventions

- Keep the diff small. One feature or fix per PR.
- UI text stays in English (the whole app is in English).
- Commit messages follow the repo's existing style: short, in Portuguese (see `git log`).
- New SVG icons follow the existing style (lucide-style, matching the palette).

## Reporting issues

- Bugs: use the **Bug report** template.
- Ideas: use the **Feature request** template.

That's it. Nothing else is required.
