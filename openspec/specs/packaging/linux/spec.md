# packaging/linux Specification

## Purpose
Packages the application for Linux distribution as a .deb package and AppImage.

## Requirements

### Requirement: .deb package
The system SHALL build a .deb package via cargo-deb with correct metadata, dependencies, and asset placement.

#### Scenario: .deb metadata
- **WHEN** the .deb package is built
- **THEN** it includes the binary at `usr/bin/appsettingstoenv`
- **THEN** it includes the desktop file at `usr/share/applications/`
- **THEN** it includes the SVG icon at `usr/share/icons/hicolor/scalable/apps/`
- **THEN** it includes the AppStream metainfo at `usr/share/metainfo/`

#### Scenario: .deb dependencies
- **WHEN** the .deb package is built
- **THEN** it declares runtime dependencies on libx11-6, libxkbcommon0, libwayland-client0, libgl1

### Requirement: AppImage package
The system SHALL bundle the application as an AppImage using appimage-bundle.

#### Scenario: AppImage is self-contained
- **WHEN** the AppImage is built
- **THEN** it runs on most Linux distributions without additional installation

### Requirement: CI builds packages
The GitHub Actions workflow SHALL build both .deb and AppImage artifacts on tagged releases.

#### Scenario: Release trigger
- **WHEN** a tag matching `v*` is pushed
- **THEN** the CI workflow builds .deb and AppImage artifacts
- **THEN** artifacts are attached to the GitHub release
