//! Ciclo de vida de la contraseña maestra: creación, desbloqueo, bloqueo,
//! rotación y (opcional) recordar la clave en el llavero del SO.

use std::fs;

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use serde::Serialize;
use tauri::{AppHandle, Manager};
use zeroize::Zeroizing;

use crate::crypto::{self, KdfParams, KEY_LEN};
use crate::models::vault::Vault;
use crate::models::vault_file::VaultOnDisk;
use crate::services::settings_service::SettingsService;
use crate::session::VaultSession;
use crate::storage::storage::Storage;

const KEYRING_SERVICE: &str = "com.user.passcontroller";
const KEYRING_ACCOUNT: &str = "vault-key";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultStatus {
    pub initialized: bool,
    pub locked: bool,
    pub has_keychain: bool,
    pub remember_device: bool,
}

pub struct AuthService;

impl AuthService {
    pub fn status(app: &AppHandle) -> Result<VaultStatus, String> {
        let session = app.state::<VaultSession>();

        // Un vault ya cifrado en disco cuenta como inicializado aunque la sesión
        // todavía no haya cargado sus parámetros.
        let initialized = match Storage::exists(app)? {
            true => matches!(Storage::read_on_disk(app)?, VaultOnDisk::Encrypted(_))
                || session.is_initialized(),
            false => session.is_initialized(),
        };

        let settings = SettingsService::load(app)?;

        Ok(VaultStatus {
            initialized,
            locked: session.is_locked(),
            has_keychain: keychain_get().is_ok(),
            remember_device: settings.remember_device,
        })
    }

    /// Primer arranque: define la contraseña maestra y migra el vault legado si
    /// lo hay, sin destruir el archivo original.
    pub fn create_master(app: &AppHandle, password: &str) -> Result<(), String> {
        validate_password(password)?;

        if Storage::exists(app)? {
            if let VaultOnDisk::Encrypted(_) = Storage::read_on_disk(app)? {
                return Err("El vault ya tiene contraseña maestra.".to_string());
            }
        }

        // Vault de partida: el legado si existe, o uno vacío.
        let starting_vault: Vault = if Storage::exists(app)? {
            match Storage::read_on_disk(app)? {
                VaultOnDisk::Legacy(v) => v,
                VaultOnDisk::Encrypted(_) => unreachable!(),
            }
        } else {
            Vault { nodes: Vec::new(), items: Vec::new() }
        };

        let params = KdfParams::default();
        let salt = crypto::random_salt();
        let key = crypto::derive_key(password, &salt, &params)?;

        let vault_path = Storage::vault_path(app)?;

        // 1) Copia intacta del archivo legado (una sola vez).
        if vault_path.exists() {
            let backup = Storage::legacy_backup_path(app)?;
            if !backup.exists() {
                fs::copy(&vault_path, &backup).map_err(|e| e.to_string())?;
            }
        }

        // 2) Escritura atómica del envelope cifrado.
        Storage::write_encrypted(&vault_path, &starting_vault, &key, &salt, &params)?;

        // 3) Verificación de ida y vuelta: releer, descifrar y comparar.
        match Storage::read_on_disk(app)? {
            VaultOnDisk::Encrypted(file) => {
                let decrypted = Storage::decrypt_file(&file, &key)?;
                if !same_vault(&starting_vault, &decrypted) {
                    return Err(
                        "La verificación de la migración falló; el vault original quedó intacto."
                            .to_string(),
                    );
                }
            }
            VaultOnDisk::Legacy(_) => {
                return Err("La migración no se aplicó correctamente.".to_string());
            }
        }

        Self::activate_session(app, key, params, salt)?;
        Ok(())
    }

    pub fn unlock(app: &AppHandle, password: &str, remember: bool) -> Result<(), String> {
        let file = match Storage::read_on_disk(app)? {
            VaultOnDisk::Encrypted(f) => f,
            VaultOnDisk::Legacy(_) => {
                return Err("El vault aún no está cifrado.".to_string())
            }
        };

        let salt = file.salt_bytes()?;
        let params = file.kdf.params.clone();
        let key = crypto::derive_key(password, &salt, &params)?;

        // Verifica la contraseña descifrando el contenido.
        Storage::decrypt_file(&file, &key)?;

        if remember {
            keychain_set(&key)?;
        }
        SettingsService::set_remember_device(app, remember)?;

        Self::activate_session(app, key, params, salt)?;
        Ok(())
    }

    pub fn unlock_with_keychain(app: &AppHandle) -> Result<(), String> {
        let key = keychain_get()?;

        let file = match Storage::read_on_disk(app)? {
            VaultOnDisk::Encrypted(f) => f,
            VaultOnDisk::Legacy(_) => return Err("El vault aún no está cifrado.".to_string()),
        };

        Storage::decrypt_file(&file, &key)?;

        let salt = file.salt_bytes()?;
        let params = file.kdf.params.clone();
        Self::activate_session(app, key, params, salt)?;
        Ok(())
    }

    pub fn lock(app: &AppHandle) {
        app.state::<VaultSession>().lock();
    }

    pub fn change_master(app: &AppHandle, current: &str, new: &str) -> Result<(), String> {
        validate_password(new)?;

        let file = match Storage::read_on_disk(app)? {
            VaultOnDisk::Encrypted(f) => f,
            VaultOnDisk::Legacy(_) => return Err("El vault aún no está cifrado.".to_string()),
        };

        let old_salt = file.salt_bytes()?;
        let old_params = file.kdf.params.clone();
        let old_key = crypto::derive_key(current, &old_salt, &old_params)?;

        let vault = Storage::decrypt_file(&file, &old_key)
            .map_err(|_| "La contraseña actual no es correcta.".to_string())?;

        let params = KdfParams::default();
        let salt = crypto::random_salt();
        let new_key = crypto::derive_key(new, &salt, &params)?;

        Storage::write_encrypted(&Storage::vault_path(app)?, &vault, &new_key, &salt, &params)?;

        // Mantén el llavero en sincronía si estaba activo.
        if keychain_get().is_ok() {
            keychain_set(&new_key)?;
        }

        Self::activate_session(app, new_key, params, salt)?;
        Ok(())
    }

    pub fn forget_keychain(app: &AppHandle) -> Result<(), String> {
        let _ = keychain_delete();
        SettingsService::set_remember_device(app, false)?;
        Ok(())
    }

    fn activate_session(
        app: &AppHandle,
        key: Zeroizing<[u8; KEY_LEN]>,
        params: KdfParams,
        salt: Vec<u8>,
    ) -> Result<(), String> {
        let session = app.state::<VaultSession>();
        session.unlock(key, params, salt);

        let settings = SettingsService::load(app)?;
        session.set_auto_lock(settings.auto_lock_seconds);
        Ok(())
    }
}

fn validate_password(password: &str) -> Result<(), String> {
    if password.chars().count() < 8 {
        return Err("La contraseña maestra debe tener al menos 8 caracteres.".to_string());
    }
    Ok(())
}

fn same_vault(a: &Vault, b: &Vault) -> bool {
    if a.nodes.len() != b.nodes.len() || a.items.len() != b.items.len() {
        return false;
    }
    let a_nodes: Vec<&String> = a.nodes.iter().map(|n| &n.id).collect();
    let b_nodes: Vec<&String> = b.nodes.iter().map(|n| &n.id).collect();
    let a_items: Vec<&String> = a.items.iter().map(|i| &i.id).collect();
    let b_items: Vec<&String> = b.items.iter().map(|i| &i.id).collect();
    a_nodes == b_nodes && a_items == b_items
}

// --- Llavero del SO ----------------------------------------------------------

fn keychain_entry() -> Result<keyring::Entry, String> {
    keyring::Entry::new(KEYRING_SERVICE, KEYRING_ACCOUNT).map_err(|e| e.to_string())
}

fn keychain_set(key: &[u8; KEY_LEN]) -> Result<(), String> {
    keychain_entry()?
        .set_password(&B64.encode(key))
        .map_err(|e| e.to_string())
}

fn keychain_get() -> Result<Zeroizing<[u8; KEY_LEN]>, String> {
    let raw = keychain_entry()?.get_password().map_err(|e| e.to_string())?;
    let bytes = B64.decode(raw).map_err(|e| e.to_string())?;
    if bytes.len() != KEY_LEN {
        return Err("Clave del llavero con tamaño inválido".to_string());
    }
    let mut out = Zeroizing::new([0u8; KEY_LEN]);
    out.copy_from_slice(&bytes);
    Ok(out)
}

fn keychain_delete() -> Result<(), String> {
    keychain_entry()?.delete_credential().map_err(|e| e.to_string())
}
