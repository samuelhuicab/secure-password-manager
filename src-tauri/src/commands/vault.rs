use tauri::AppHandle;

use crate::{
    models::vault::Vault,
    services::{backup_service::BackupService, vault_service::VaultService},
};

#[tauri::command]
pub fn load_vault(app: AppHandle) -> Result<Vault, String> {
    VaultService::load(&app)
}

#[tauri::command]
pub fn save_vault(app: AppHandle, vault: Vault) -> Result<(), String> {
    VaultService::save(&app, &vault)
}

#[tauri::command]
pub fn export_backup(app: AppHandle, path: String, password: String) -> Result<(), String> {
    BackupService::export(&app, &path, &password)
}

#[tauri::command]
pub fn import_backup(app: AppHandle, path: String, password: String) -> Result<(), String> {
    BackupService::import(&app, &path, &password)
}
