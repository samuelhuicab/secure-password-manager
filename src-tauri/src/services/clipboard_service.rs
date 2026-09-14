//! Copiado de secretos al portapapeles con borrado automático.

use std::thread;
use std::time::Duration;

use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::services::vault_service::VaultService;

pub struct ClipboardService;

impl ClipboardService {
    /// Escribe `value` en el portapapeles y, si `clear_after > 0`, lo limpia
    /// pasados esos segundos (solo si su contenido no cambió mientras tanto).
    pub fn copy(app: &AppHandle, value: String, clear_after: u64) -> Result<(), String> {
        app.clipboard()
            .write_text(value.clone())
            .map_err(|e| e.to_string())?;

        if clear_after > 0 {
            let app = app.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(clear_after));
                if let Ok(current) = app.clipboard().read_text() {
                    if current == value {
                        let _ = app.clipboard().write_text(String::new());
                    }
                }
            });
        }

        Ok(())
    }

    /// Copia `KEY=VALUE` en formato `export` de shell.
    pub fn copy_as_export(
        app: &AppHandle,
        key: String,
        value: String,
        clear_after: u64,
    ) -> Result<(), String> {
        let key = key
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect::<String>()
            .to_uppercase();
        let line = format!("export {key}=\"{}\"", value.replace('"', "\\\""));
        Self::copy(app, line, clear_after)
    }

    /// Construye y copia una cadena de conexión a partir de los campos del item.
    pub fn copy_connection_string(
        app: &AppHandle,
        item_id: String,
        clear_after: u64,
    ) -> Result<String, String> {
        let vault = VaultService::load(app)?;
        let item = vault
            .items
            .iter()
            .find(|i| i.id == item_id)
            .ok_or("Item no encontrado")?;

        let get = |k: &str| {
            item.fields
                .iter()
                .find(|f| f.key == k)
                .map(|f| f.value.clone())
                .unwrap_or_default()
        };

        let conn = match item.item_type {
            crate::models::item_type::ItemType::Database => {
                let host = or(get("host"), "localhost");
                let user = get("username");
                let pass = get("password");
                let db = get("database");
                let port = get("port");
                let scheme = guess_db_scheme(&port, &db);
                let auth = if pass.is_empty() {
                    user.clone()
                } else {
                    format!("{user}:{pass}")
                };
                let port = if port.is_empty() {
                    String::new()
                } else {
                    format!(":{port}")
                };
                format!("{scheme}://{auth}@{host}{port}/{db}")
            }
            crate::models::item_type::ItemType::Server => {
                let host = get("host");
                let user = get("username");
                let port = get("port");
                let p = if port.is_empty() {
                    String::new()
                } else {
                    format!(" -p {port}")
                };
                format!("ssh {user}@{host}{p}")
            }
            _ => return Err("Este tipo de item no tiene cadena de conexión.".to_string()),
        };

        Self::copy(app, conn.clone(), clear_after)?;
        Ok(conn)
    }
}

fn or(value: String, fallback: &str) -> String {
    if value.is_empty() {
        fallback.to_string()
    } else {
        value
    }
}

fn guess_db_scheme(port: &str, db: &str) -> &'static str {
    match port {
        "3306" => "mysql",
        "27017" => "mongodb",
        "5432" => "postgresql",
        _ => {
            if db.to_lowercase().contains("mongo") {
                "mongodb"
            } else {
                "postgresql"
            }
        }
    }
}
