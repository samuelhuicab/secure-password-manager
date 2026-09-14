use std::fs;
use std::path::{Path, PathBuf};

use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

use serde_json::{from_slice, to_string_pretty, to_vec};

use crate::crypto::{self, KdfParams};
use crate::models::vault::Vault;
use crate::models::vault_file::{self, VaultFile, VaultOnDisk};
use crate::session::VaultSession;

pub struct Storage;

impl Storage {
    pub fn vault_path(app: &AppHandle) -> Result<PathBuf, String> {
        let path = app
            .path()
            .resolve("vault.json", BaseDirectory::AppConfig)
            .map_err(|e| e.to_string())?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        Ok(path)
    }

    /// Copia de seguridad, hecha una sola vez, del vault en texto plano antes de
    /// migrarlo a formato cifrado.
    pub fn legacy_backup_path(app: &AppHandle) -> Result<PathBuf, String> {
        Ok(Self::vault_path(app)?.with_file_name("vault.v1.bak.json"))
    }

    pub fn exists(app: &AppHandle) -> Result<bool, String> {
        Ok(Self::vault_path(app)?.exists())
    }

    pub fn read_on_disk(app: &AppHandle) -> Result<VaultOnDisk, String> {
        let raw = fs::read_to_string(Self::vault_path(app)?).map_err(|e| e.to_string())?;
        vault_file::parse(&raw)
    }

    /// Escribe el envelope cifrado de `vault` de forma atómica (tmp + rename).
    pub fn write_encrypted(
        path: &Path,
        vault: &Vault,
        key: &[u8; crypto::KEY_LEN],
        salt: &[u8],
        kdf: &KdfParams,
    ) -> Result<(), String> {
        let plaintext = to_vec(vault).map_err(|e| e.to_string())?;
        let (nonce, ciphertext) = crypto::encrypt(key, &plaintext)?;

        let file = VaultFile::new(kdf, salt, &nonce, &ciphertext);
        let json = to_string_pretty(&file).map_err(|e| e.to_string())?;

        atomic_write(path, json.as_bytes())
    }

    /// Descifra y deserializa un envelope con la clave dada.
    pub fn decrypt_file(file: &VaultFile, key: &[u8; crypto::KEY_LEN]) -> Result<Vault, String> {
        let nonce = file.nonce_bytes()?;
        let ciphertext = file.ciphertext_bytes()?;
        let plaintext = crypto::decrypt(key, &nonce, &ciphertext)?;
        from_slice(&plaintext).map_err(|e| format!("Vault corrupto: {e}"))
    }

    // --- API que usan los servicios (misma firma de siempre) ------------------

    pub fn load(app: &AppHandle) -> Result<Vault, String> {
        let session = app.state::<VaultSession>();
        let key = session.require_key()?;

        match Self::read_on_disk(app)? {
            VaultOnDisk::Encrypted(file) => Self::decrypt_file(&file, &key),
            VaultOnDisk::Legacy(_) => {
                Err("El vault todavía no está cifrado. Reinicia la app para migrarlo.".to_string())
            }
        }
    }

    pub fn save(app: &AppHandle, vault: &Vault) -> Result<(), String> {
        let session = app.state::<VaultSession>();
        let key = session.require_key()?;

        let (salt, kdf) = {
            let inner = session.0.lock().unwrap();
            (inner.salt.clone(), inner.kdf.clone())
        };

        Self::write_encrypted(&Self::vault_path(app)?, vault, &key, &salt, &kdf)
    }
}

/// Escritura atómica: se escribe en `<path>.tmp` y luego se renombra encima del
/// destino. Un corte a mitad de operación nunca deja el vault a medias.
pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    fs::write(&tmp, bytes).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::vault_file::{parse, VaultOnDisk};

    fn tmp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "passcontroller-test-{tag}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    const LEGACY: &str = r#"{
        "nodes": [{"id":"n1","name":"Dev","parent_id":null,"node_type":"Folder","icon":null,"order":0,"expanded":true}],
        "items": [{"id":"i1","node_id":"n1","title":"SSH","item_type":"Server","favorite":false,"notes":"","created_at":"x","updated_at":"x","fields":[{"id":"f1","key":"password","label":"Contraseña","value":"s3cr3t","hidden":true}]}]
    }"#;

    #[test]
    fn migrates_legacy_vault_without_data_loss() {
        let dir = tmp_dir("migrate");
        let path = dir.join("vault.json");
        fs::write(&path, LEGACY).unwrap();

        // Vault de partida (como haría create_master).
        let legacy = match parse(LEGACY).unwrap() {
            VaultOnDisk::Legacy(v) => v,
            _ => panic!("debería ser legado"),
        };

        // Copia de seguridad + cifrado atómico.
        let backup = path.with_file_name("vault.v1.bak.json");
        fs::copy(&path, &backup).unwrap();

        let params = KdfParams::default();
        let salt = crypto::random_salt();
        let key = crypto::derive_key("clave-maestra-test", &salt, &params).unwrap();
        Storage::write_encrypted(&path, &legacy, &key, &salt, &params).unwrap();

        // El backup conserva el texto plano original.
        assert_eq!(fs::read_to_string(&backup).unwrap(), LEGACY);

        // Releer + descifrar => mismo contenido.
        let reloaded = match parse(&fs::read_to_string(&path).unwrap()).unwrap() {
            VaultOnDisk::Encrypted(f) => Storage::decrypt_file(&f, &key).unwrap(),
            _ => panic!("debería estar cifrado"),
        };
        assert_eq!(reloaded.nodes.len(), 1);
        assert_eq!(reloaded.items.len(), 1);
        assert_eq!(reloaded.items[0].fields[0].value, "s3cr3t");

        // Con clave equivocada no se puede abrir.
        let bad = crypto::derive_key("otra", &salt, &params).unwrap();
        if let VaultOnDisk::Encrypted(f) = parse(&fs::read_to_string(&path).unwrap()).unwrap() {
            assert!(Storage::decrypt_file(&f, &bad).is_err());
        }

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn atomic_write_replaces_contents() {
        let dir = tmp_dir("atomic");
        let path = dir.join("data.json");
        atomic_write(&path, b"uno").unwrap();
        atomic_write(&path, b"dos").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "dos");
        assert!(!path.with_file_name("data.json.tmp").exists());
        fs::remove_dir_all(&dir).ok();
    }
}
