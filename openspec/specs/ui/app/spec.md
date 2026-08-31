# ui/app Specification

## Purpose
Provides the desktop GUI for real-time JSON-to-ENV conversion with a two-pane layout, theme system, and about dialog.

## Requirements

### Requirement: Two-pane layout
The system SHALL display a fixed 1280x800 window with two side-by-side panes: input (JSON) on the left and output (ENV) on the right.

#### Scenario: Window dimensions
- **WHEN** the application launches
- **THEN** the window is 1280x800 pixels and cannot be resized

### Requirement: Real-time conversion
The system SHALL update the output pane immediately on every keystroke or text change in the input pane.

#### Scenario: Typing triggers conversion
- **WHEN** the user types a character in the input pane
- **THEN** the output pane updates with the conversion result

### Requirement: Input pane controls
The input pane SHALL provide a "Load JSON" button (file upload) and a "Clear" button.

#### Scenario: Upload button present
- **WHEN** the input pane is displayed
- **THEN** a "Load JSON" icon button is visible in the input pane footer

#### Scenario: Clear button present
- **WHEN** the input pane is displayed
- **THEN** a "Clear" icon button is visible in the input pane footer

### Requirement: Output pane controls
The output pane SHALL provide a "Copy" button and a "Save as .env" button.

#### Scenario: Copy button present
- **WHEN** the output pane is displayed
- **THEN** a "Copy" icon button is visible in the output pane footer

#### Scenario: Save button present
- **WHEN** the output pane is displayed
- **THEN** a "Save as .env" icon button is visible in the output pane footer

### Requirement: Variable count display
The system SHALL display the number of converted variables below the output pane when the count is greater than zero.

#### Scenario: Count shown after conversion
- **WHEN** conversion produces 3 variables
- **THEN** the output pane footer displays "3 variables"

#### Scenario: No count on empty output
- **WHEN** output is empty
- **THEN** no variable count is displayed

### Requirement: Theme toggle
The system SHALL provide a theme toggle button that switches between light and dark color schemes.

#### Scenario: Toggle from dark to light
- **WHEN** the theme is dark and the user clicks the theme toggle
- **THEN** the application switches to the light color scheme

#### Scenario: Toggle from light to dark
- **WHEN** the theme is light and the user clicks the theme toggle
- **THEN** the application switches to the dark color scheme

### Requirement: About popup
The system SHALL provide an "About" button that opens a popup with project info, author name, and links to GitHub and LinkedIn.

#### Scenario: Open about popup
- **WHEN** the user clicks the info icon button
- **THEN** an about popup appears centered in the window

#### Scenario: Close about popup
- **WHEN** the user clicks outside the about popup
- **THEN** the popup closes

### Requirement: Status messages
The system SHALL display transient status messages (e.g., "Copied!", "Saved!", error messages) in the footer area.

#### Scenario: Copy success status
- **WHEN** the user clicks Copy and clipboard write succeeds
- **THEN** the output pane footer displays "Copied!"

#### Scenario: Copy failure status
- **WHEN** the clipboard write fails
- **THEN** the output pane footer displays the error message
