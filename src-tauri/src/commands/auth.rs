use tauri::{AppHandle, Manager};

use crate::services::auth_service::{AuthService, VaultStatus};

#[tauri::command]
pub fn vault_status(app: AppHandle) -> Result<VaultStatus, String> {
    AuthService::status(&app)
}

#[tauri::command]
pub fn create_master_password(app: AppHandle, password: String) -> Result<(), String> {
    AuthService::create_master(&app, &password)
}

#[tauri::command]
pub fn unlock_vault(app: AppHandle, password: String, remember: bool) -> Result<(), String> {
    AuthService::unlock(&app, &password, remember)
}

#[tauri::command]
pub fn unlock_with_keychain(app: AppHandle) -> Result<(), String> {
    AuthService::unlock_with_keychain(&app)
}

#[tauri::command]
pub fn lock_vault(app: AppHandle) {
    AuthService::lock(&app);
}

/// El frontend la llama al detectar actividad para que el contador de
/// inactividad del backend no se adelante al de la UI.
#[tauri::command]
pub fn touch_activity(app: AppHandle) {
    app.state::<crate::session::VaultSession>().touch();
}

#[tauri::command]
pub fn change_master_password(
    app: AppHandle,
    current: String,
    new: String,
) -> Result<(), String> {
    AuthService::change_master(&app, &current, &new)
}

#[tauri::command]
pub fn forget_keychain(app: AppHandle) -> Result<(), String> {
    AuthService::forget_keychain(&app)
}
