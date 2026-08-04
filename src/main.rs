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
        serde_json::Value::Null => {} // null nao pode ser armazenado em config
        scalar => {
            // ponytail: valores crus, sem desescapar
            let value = scalar.as_str().map(String::from).unwrap_or_else(|| scalar.to_string());
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
        Err(e) => e.to_string(), // painel direito mostra o erro do parser
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
    // ponytail: clipboard persistente, senha o X11 manager nao ve o conteudo
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
    // ponytail: file picker bloqueia a thread -> roda em thread propria, volta via event loop
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
                None => return, // cancelado pelo usuario
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
