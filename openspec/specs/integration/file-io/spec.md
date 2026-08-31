# integration/file-io Specification

## Purpose
Provides file I/O operations: loading JSON files into the input pane and saving converted output to .env files.

## Requirements

### Requirement: Upload JSON file
The system SHALL open a file dialog filtered to `.json` files and load the selected file's content into the input pane.

#### Scenario: File selected
- **WHEN** the user selects a `.json` file in the file dialog
- **THEN** the file content replaces the input pane text
- **THEN** the conversion runs automatically on the loaded content

#### Scenario: File read error
- **WHEN** the selected file cannot be read
- **THEN** the input pane is cleared
- **THEN** an error status is displayed

#### Scenario: User cancels dialog
- **WHEN** the user cancels the file dialog
- **THEN** no action is taken

### Requirement: Save output as .env file
The system SHALL open a save dialog pre-filled with `appsettings.env` and write the output text to the selected path.

#### Scenario: Successful save
- **WHEN** the user selects a save location
- **THEN** the output text is written to the file
- **THEN** a "Saved!" status is displayed

#### Scenario: Save with empty output
- **WHEN** the output is empty and the user clicks Save
- **THEN** the status displays "Nothing to save"
- **THEN** no file dialog opens

#### Scenario: Save error
- **WHEN** writing the file fails
- **THEN** the status displays "Save failed: <error>"

### Requirement: File dialog runs off main thread
The system SHALL run file dialogs on a background thread to avoid blocking the UI.

#### Scenario: UI remains responsive during dialog
- **WHEN** a file dialog is open
- **THEN** the rest of the UI remains responsive
