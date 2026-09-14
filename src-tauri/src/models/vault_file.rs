//! Formato en disco de `vault.json`.
//!
//! - v1 (legado): el `Vault` serializado en texto plano, sin campo `version`.
//! - v2: envelope cifrado con la clave derivada de la contraseña maestra.

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use serde::{Deserialize, Serialize};

use crate::crypto::KdfParams;
use crate::models::vault::Vault;

pub const CURRENT_VERSION: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdfEnvelope {
    #[serde(flatten)]
    pub params: KdfParams,
    /// Salt en base64.
    pub salt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultFile {
    pub version: u32,
    pub kdf: KdfEnvelope,
    pub cipher: String,
    /// Nonce en base64.
    pub nonce: String,
    /// Texto cifrado en base64.
    pub ciphertext: String,
}

impl VaultFile {
    pub fn new(params: &KdfParams, salt: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Self {
        Self {
            version: CURRENT_VERSION,
            kdf: KdfEnvelope {
                params: params.clone(),
                salt: B64.encode(salt),
            },
            cipher: "xchacha20poly1305".to_string(),
            nonce: B64.encode(nonce),
            ciphertext: B64.encode(ciphertext),
        }
    }

    pub fn salt_bytes(&self) -> Result<Vec<u8>, String> {
        B64.decode(&self.kdf.salt).map_err(|e| e.to_string())
    }

    pub fn nonce_bytes(&self) -> Result<Vec<u8>, String> {
        B64.decode(&self.nonce).map_err(|e| e.to_string())
    }

    pub fn ciphertext_bytes(&self) -> Result<Vec<u8>, String> {
        B64.decode(&self.ciphertext).map_err(|e| e.to_string())
    }
}

/// Resultado de leer el archivo del vault desde disco.
pub enum VaultOnDisk {
    /// Vault en texto plano de una versión anterior de la app.
    Legacy(Vault),
    /// Vault cifrado.
    Encrypted(VaultFile),
}

/// Interpreta el contenido bruto de `vault.json`.
pub fn parse(raw: &str) -> Result<VaultOnDisk, String> {
    let value: serde_json::Value =
        serde_json::from_str(raw).map_err(|e| format!("vault.json ilegible: {e}"))?;

    let looks_encrypted = value.get("version").is_some() && value.get("ciphertext").is_some();

    if looks_encrypted {
        let file: VaultFile =
            serde_json::from_value(value).map_err(|e| format!("Envelope inválido: {e}"))?;
        Ok(VaultOnDisk::Encrypted(file))
    } else {
        let vault: Vault =
            serde_json::from_value(value).map_err(|e| format!("Vault legado inválido: {e}"))?;
        Ok(VaultOnDisk::Legacy(vault))
    }
}
