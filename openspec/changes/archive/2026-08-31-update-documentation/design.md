## Context

README.md is 49 lines: usage, conversion rules, `cargo run`, `cargo test`, stack list. CONTRIBUTING.md is 34 lines. No installation instructions, no build prerequisites, no packaging docs. See proposal.md for motivation.

## Goals / Non-Goals

**Goals:**
- Make the README a single entry point for users, packagers, and contributors
- Keep it scannable — sections for different audiences (users, packagers, contributors)
- Preserve existing content (conversion rules table, stack list)

**Non-Goals:**
- Adding screenshots (no images available yet — leave placeholders)
- Creating a separate docs site
- Translating docs

## Decisions

### README restructuring: task-based sections
Split by what the reader wants to DO: install, build, use, contribute. Each section starts with a one-liner.

**Why**: Readers arrive with different goals. A task-based structure lets them skip to what they need.

### Installation as the first section after the intro
Put `cargo install`, .deb, AppImage, and Flatpak before "How to run" (which becomes "Build from source").

**Why**: Most users want to install, not build. `cargo install appsettingstoenv` should be the first command they see.

### CONTRIBUTING.md: add development environment setup
Add system dependencies (Rust, Slint prereqs, pkg-config), IDE suggestions, and the full check pipeline.

**Why**: New contributors currently have to guess what's needed. The CI checks (fmt, clippy, test) should be documented with the exact commands.

### Keep everything in two files
No new files. README for users/packagers, CONTRIBUTING for developers.

**Why**: Small project, small docs. A separate ARCHITECTURE.md or INSTALL.md would be overkill.

## Risks / Trade-offs

- **README gets longer** → Acceptable if scannable (headings, short paragraphs, code blocks). Can always add a TOC later.
- **Flatpak instructions may drift** → Mitigated by linking to the CI workflow as source of truth for build steps.
