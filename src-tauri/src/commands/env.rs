use tauri::AppHandle;

use crate::models::item::VaultItem;
use crate::services::env_service::EnvService;

#[tauri::command]
pub fn export_env(
    app: AppHandle,
    item_id: String,
    path: Option<String>,
) -> Result<String, String> {
    let content = EnvService::export(&app, &item_id)?;
    if let Some(path) = path {
        std::fs::write(&path, &content).map_err(|e| e.to_string())?;
    }
    Ok(content)
}

#[tauri::command]
pub fn import_env(
    app: AppHandle,
    node_id: String,
    title: String,
    content: Option<String>,
    path: Option<String>,
) -> Result<VaultItem, String> {
    let content = match (content, path) {
        (Some(c), _) => c,
        (None, Some(p)) => std::fs::read_to_string(&p).map_err(|e| e.to_string())?,
        (None, None) => return Err("Falta el contenido o la ruta del archivo.".to_string()),
    };
    EnvService::import(&app, &node_id, &title, &content)
}
