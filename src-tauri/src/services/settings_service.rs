//! Carga y guardado de `settings.json` (preferencias no sensibles).

use std::fs;
use std::path::PathBuf;

use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

use crate::models::settings::Settings;
use crate::session::VaultSession;
use crate::storage::storage::atomic_write;

pub struct SettingsService;

impl SettingsService {
    fn path(app: &AppHandle) -> Result<PathBuf, String> {
        let path = app
            .path()
            .resolve("settings.json", BaseDirectory::AppConfig)
            .map_err(|e| e.to_string())?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        Ok(path)
    }

    pub fn load(app: &AppHandle) -> Result<Settings, String> {
        let path = Self::path(app)?;
        if !path.exists() {
            return Ok(Settings::default());
        }
        let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        // Un settings.json dañado no debe impedir abrir la app.
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    }

    pub fn save(app: &AppHandle, settings: &Settings) -> Result<(), String> {
        let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
        atomic_write(&Self::path(app)?, json.as_bytes())?;

        // Aplica el nuevo tiempo de auto-bloqueo a la sesión viva.
        app.state::<VaultSession>()
            .set_auto_lock(settings.auto_lock_seconds);
        Ok(())
    }

    pub fn set_remember_device(app: &AppHandle, value: bool) -> Result<(), String> {
        let mut settings = Self::load(app)?;
        settings.remember_device = value;
        Self::save(app, &settings)
    }
}
