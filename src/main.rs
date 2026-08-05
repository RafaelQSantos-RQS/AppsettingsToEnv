slint::include_modules!();

fn flatten(value: &serde_json::Value, prefix: &str, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}__{k}")
                };
                flatten(v, &key, out);
            }
        }
        serde_json::Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                flatten(v, &format!("{prefix}__{i}"), out);
            }
        }
        serde_json::Value::Null => {} // null can't be stored in config
        scalar => {
            // Escape line breaks so the value stays one line (valid for .env files)
            let value = scalar
                .as_str()
                .map(|s| s.replace('\r', "\\r").replace('\n', "\\n").replace('\t', "\\t"))
                .unwrap_or_else(|| scalar.to_string());
            out.push(format!("{prefix}={value}"));
        }
    }
}

fn convert(input: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(root) => {
            let mut out = Vec::new();
            flatten(&root, "", &mut out);
            out.join("\n")
        }
        Err(e) => e.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::convert;

    #[test]
    fn flattens_nested_arrays_and_skips_null() {
        let json = r#"{
            "ConnectionStrings": { "Default": "Server=db" },
            "AllowedHosts": ["a", "b"],
            "Feature": null
        }"#;
        let out = convert(json);
        assert!(out.contains("ConnectionStrings__Default=Server=db"));
        assert!(out.contains("AllowedHosts__0=a"));
        assert!(out.contains("AllowedHosts__1=b"));
        assert!(!out.contains("Feature"));
    }

    #[test]
    fn escapes_line_breaks_and_tabs() {
        let json = r#"{"Log": "Line 1\nLine 2\r\nLine 3\tTab", "Path": "C:\\data"}"#;
        let out = convert(json);
        assert!(out.contains("Log=Line 1\\nLine 2\\r\\nLine 3\\tTab"));
        assert!(out.contains("Path=C:\\data"));
    }

    #[test]
    fn reports_parse_error() {
        assert!(convert("{ not json").contains("line"));
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    let handle = ui.as_weak();
    ui.on_input_changed(move |text| {
        let out = convert(&text);
        if let Some(ui) = handle.upgrade() {
            ui.set_output(out.into());
        }
    });

    let handle = ui.as_weak();
    // Persistent clipboard: a dropped Clipboard is never seen by the X11 manager.
    let mut clipboard = arboard::Clipboard::new();
    ui.on_copy_requested(move || {
        let Some(ui) = handle.upgrade() else { return };
        let text = ui.get_output().to_string();
        let status = match clipboard.as_mut() {
            Ok(cb) => match cb.set_text(text) {
                Ok(()) => "Copiado!".to_string(),
                Err(e) => format!("Falha ao copiar: {e}"),
            },
            Err(e) => format!("Sem clipboard: {e}"),
        };
        ui.set_copy_status(status.into());
    });

    let handle = ui.as_weak();
    ui.on_clear_requested(move || {
        if let Some(ui) = handle.upgrade() {
            ui.set_input("".into());
            ui.set_output("".into());
            ui.set_copy_status("".into());
        }
    });

    let handle = ui.as_weak();
    // The file dialog blocks its thread, so it runs on a worker thread.
    ui.on_upload_requested(move || {
        let handle = handle.clone();
        std::thread::spawn(move || {
            let picked = rfd::FileDialog::new()
                .add_filter("JSON", &["json"])
                .pick_file();
            let (content, status) = match picked {
                Some(path) => match std::fs::read_to_string(&path) {
                    Ok(c) => (c, String::new()),
                    Err(e) => (String::new(), format!("Erro ao ler arquivo: {e}")),
                },
                None => return, // user cancelled
            };
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = handle.upgrade() {
                    ui.set_input(content.clone().into());
                    ui.set_output(convert(&content).into());
                    ui.set_upload_status(status.clone().into());
                }
            });
        });
    });

    ui.run()
}
