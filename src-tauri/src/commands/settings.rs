use tauri::AppHandle;

use crate::models::settings::Settings;
use crate::services::settings_service::SettingsService;

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<Settings, String> {
    SettingsService::load(&app)
}

#[tauri::command]
pub fn update_settings(app: AppHandle, settings: Settings) -> Result<Settings, String> {
    SettingsService::save(&app, &settings)?;
    Ok(settings)
}
