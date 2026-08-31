## 1. Update README.md

- [x] 1.1 Add installation section after intro with `cargo install`, .deb, AppImage, and Flatpak options — verify the section exists and each method has a code block
- [x] 1.2 Rename "How to run" to "Build from source" and add prerequisites (Rust, system deps for Slint) — verify prerequisites are listed before `cargo run`
- [x] 1.3 Add a troubleshooting/FAQ section with at least 2 common issues (Slint build fails, clipboard doesn't work on Wayland) — verify each entry has a problem + solution format
- [x] 1.4 Add a screenshots placeholder section with a comment for future images — verify section exists between intro and usage

## 2. Update CONTRIBUTING.md

- [x] 2.1 Add development environment setup section with system dependencies (Rust, pkg-config, libx11, libxkbcommon, libwayland, libgl) — verify each dep is listed with install hint
- [x] 2.2 Add IDE recommendations (VS Code + rust-analyzer) — verify section exists

## 3. Verify

- [x] 3.1 Review both files for broken markdown (headers, links, code blocks) — verify `grep -c '```' README.md` returns an even number and same for CONTRIBUTING.md
