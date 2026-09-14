use tauri::AppHandle;

use crate::services::clipboard_service::ClipboardService;

#[tauri::command]
pub fn copy_secret(app: AppHandle, value: String, clear_after: u64) -> Result<(), String> {
    ClipboardService::copy(&app, value, clear_after)
}

#[tauri::command]
pub fn copy_as_export(
    app: AppHandle,
    key: String,
    value: String,
    clear_after: u64,
) -> Result<(), String> {
    ClipboardService::copy_as_export(&app, key, value, clear_after)
}

#[tauri::command]
pub fn copy_connection_string(
    app: AppHandle,
    item_id: String,
    clear_after: u64,
) -> Result<String, String> {
    ClipboardService::copy_connection_string(&app, item_id, clear_after)
}
