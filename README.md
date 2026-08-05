# Appsettings To Env

[![CI](https://github.com/RafaelQSantos-RQS/AppsettingsToEnv/actions/workflows/ci.yml/badge.svg)](https://github.com/RafaelQSantos-RQS/AppsettingsToEnv/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Converts `appsettings.json` (.NET configuration) into environment variables in `KEY=value` format, ready for Docker containers.

## Usage

Paste the JSON into the left panel — the conversion happens in real time on the right panel. Then copy the result.

- **Upload** — loads a `.json` file to replace the left panel content
- **Clear** — empties both panels
- **Copy** — copies the result to the clipboard
- **Save as .env** — saves the result to a `.env` file
- **Theme** — toggles between light and dark themes

## Conversion rules

Follows the Microsoft convention for configuration via environment variables:

| JSON | Result |
|---|---|
| `"ConnectionStrings": { "Default": "Server=db" }` | `ConnectionStrings__Default=Server=db` |
| `"AllowedHosts": ["a", "b"]` | `AllowedHosts__0=a`<br>`AllowedHosts__1=b` |
| `"Feature": null` | *(omitted)* |

- Nested keys use `__` as separator
- Arrays are indexed (`Key__0`, `Key__1`, ...)
- `null` values are ignored
- Invalid JSON shows the parser error in the right panel
- Line breaks and tabs in values are escaped (`\n`, `\r`, `\t`) to keep one entry per line

## How to run

```bash
cargo run
```

## Tests

```bash
cargo test
```

## Stack

- **Rust** with [Slint](https://slint.dev) (desktop UI)
- **serde_json** (parse), **arboard** (clipboard), **rfd** (file picker)
