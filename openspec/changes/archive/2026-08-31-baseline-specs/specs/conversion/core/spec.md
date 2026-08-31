## Purpose

Converts JSON configuration (appsettings.json format) into flat KEY=VALUE environment variable lines following the Microsoft configuration provider convention.

## ADDED Requirements

### Requirement: Flatten nested objects
The system SHALL flatten nested JSON objects into dot-free environment variable keys using `__` as the separator between levels.

#### Scenario: Deeply nested object
- **WHEN** input contains `{"A": {"B": {"C": "value"}}}`
- **THEN** output contains `A__B__C=value`

#### Scenario: Top-level key
- **WHEN** input contains `{"Key": "value"}`
- **THEN** output contains `Key=value`

### Requirement: Index JSON arrays
The system SHALL convert JSON arrays into indexed keys using `__0`, `__1`, etc.

#### Scenario: Array with multiple elements
- **WHEN** input contains `{"Items": ["a", "b", "c"]}`
- **THEN** output contains `Items__0=a`, `Items__1=b`, `Items__2=c`

#### Scenario: Empty array
- **WHEN** input contains `{"Items": []}`
- **THEN** output contains no entries for `Items`

### Requirement: Skip null values
The system SHALL omit entries where the JSON value is `null`.

#### Scenario: Null value is excluded
- **WHEN** input contains `{"Feature": null}`
- **THEN** output does not contain a `Feature` entry

### Requirement: Escape control characters in values
The system SHALL escape line breaks (`\n`, `\r`) and tabs (`\t`) in string values so each entry occupies a single line.

#### Scenario: Multi-line string value
- **WHEN** input value contains a literal newline
- **THEN** output value contains `\n` (escaped)

#### Scenario: Tab character in value
- **WHEN** input value contains a literal tab
- **THEN** output value contains `\t` (escaped)

### Requirement: Preserve non-string scalars
The system SHALL represent numbers and booleans as their JSON text representation.

#### Scenario: Numeric value
- **WHEN** input contains `{"Port": 8080}`
- **THEN** output contains `Port=8080`

#### Scenario: Boolean value
- **WHEN** input contains `{"Enabled": true}`
- **THEN** output contains `Enabled=true`

### Requirement: Report invalid JSON
The system SHALL display the parser error message when the input is not valid JSON.

#### Scenario: Malformed JSON input
- **WHEN** input is not valid JSON
- **THEN** output displays the parse error description (including line/column info)

### Requirement: Count output variables
The system SHALL report the total number of KEY=VALUE lines produced by a successful conversion.

#### Scenario: Variable count after conversion
- **WHEN** input is valid JSON producing 5 entries
- **THEN** the reported variable count is 5

#### Scenario: Variable count on error
- **WHEN** input is invalid JSON
- **THEN** the reported variable count is 0
