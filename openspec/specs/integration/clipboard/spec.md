# integration/clipboard Specification

## Purpose
Provides clipboard integration for copying the converted ENV output to the system clipboard.

## Requirements

### Requirement: Copy output to clipboard
The system SHALL copy the full output text to the system clipboard when the user triggers the copy action.

#### Scenario: Successful copy
- **WHEN** the user clicks the Copy button and the output is non-empty
- **THEN** the output text is placed on the system clipboard
- **THEN** a "Copied!" status message is displayed

#### Scenario: Copy empty output
- **WHEN** the user clicks the Copy button and the output is empty
- **THEN** the clipboard is not modified

### Requirement: Handle clipboard errors
The system SHALL display an error message if the clipboard operation fails.

#### Scenario: Clipboard unavailable
- **WHEN** the clipboard cannot be initialized
- **THEN** the status displays "No clipboard: <error>"

#### Scenario: Clipboard write failure
- **WHEN** setting clipboard text fails
- **THEN** the status displays "Copy failed: <error>"
