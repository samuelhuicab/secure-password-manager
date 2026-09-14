use tauri::AppHandle;

use crate::{
    models::vault::Vault,
    storage::storage::Storage,
};

pub struct VaultService;

impl VaultService {
    pub fn load(app: &AppHandle) -> Result<Vault, String> {
        Storage::load(app)
    }

    pub fn save(app: &AppHandle, vault: &Vault) -> Result<(), String> {
        Storage::save(app, vault)
    }
}
