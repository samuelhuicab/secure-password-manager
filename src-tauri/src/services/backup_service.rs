//! Export / import de respaldos cifrados (`.vault`).
//!
//! Un backup usa el mismo envelope que `vault.json` pero con su propia
//! contraseña, para poder moverlo entre equipos.

use std::fs;
use std::path::Path;

use serde_json::to_string_pretty;

use crate::crypto::{self, KdfParams};
use crate::models::vault_file::{parse, VaultFile, VaultOnDisk};
use crate::services::vault_service::VaultService;
use crate::storage::storage::{atomic_write, Storage};

pub struct BackupService;

impl BackupService {
    /// Escribe el vault actual cifrado con `password` en `path`.
    pub fn export(app: &tauri::AppHandle, path: &str, password: &str) -> Result<(), String> {
        if password.chars().count() < 8 {
            return Err("La contraseña del backup debe tener al menos 8 caracteres.".to_string());
        }

        let vault = VaultService::load(app)?; // exige vault desbloqueado

        let params = KdfParams::default();
        let salt = crypto::random_salt();
        let key = crypto::derive_key(password, &salt, &params)?;

        let plaintext = serde_json::to_vec(&vault).map_err(|e| e.to_string())?;
        let (nonce, ciphertext) = crypto::encrypt(&key, &plaintext)?;

        let file = VaultFile::new(&params, &salt, &nonce, &ciphertext);
        let json = to_string_pretty(&file).map_err(|e| e.to_string())?;

        atomic_write(Path::new(path), json.as_bytes())
    }

    /// Lee `path`, lo descifra con `password` y REEMPLAZA el contenido del vault
    /// actual (re-cifrándolo con la clave de la sesión abierta).
    pub fn import(app: &tauri::AppHandle, path: &str, password: &str) -> Result<(), String> {
        let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;

        let file = match parse(&raw)? {
            VaultOnDisk::Encrypted(f) => f,
            VaultOnDisk::Legacy(_) => {
                return Err("El archivo no es un backup cifrado válido.".to_string())
            }
        };

        let salt = file.salt_bytes()?;
        let key = crypto::derive_key(password, &salt, &file.kdf.params)?;
        let vault = Storage::decrypt_file(&file, &key)
            .map_err(|_| "Contraseña del backup incorrecta o archivo dañado.".to_string())?;

        VaultService::save(app, &vault)
    }
}
