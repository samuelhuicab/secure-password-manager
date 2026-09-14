//! Primitivas criptográficas del vault.
//!
//! - Derivación de clave: Argon2id (parámetros OWASP por defecto).
//! - Cifrado: XChaCha20-Poly1305 (AEAD), nonce aleatorio de 24 bytes por operación.
//!
//! La clave derivada se maneja siempre dentro de `Zeroizing` para que se borre
//! de memoria al soltarse.

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305, XNonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

pub const KEY_LEN: usize = 32;
pub const NONCE_LEN: usize = 24;
pub const SALT_LEN: usize = 16;

/// Parámetros de Argon2id que se guardan junto al vault para poder subir el
/// coste en el futuro sin romper vaults viejos.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KdfParams {
    pub algo: String,
    pub m_cost: u32,
    pub t_cost: u32,
    pub p_cost: u32,
}

impl Default for KdfParams {
    fn default() -> Self {
        // OWASP: 19 MiB de memoria, 2 iteraciones, 1 grado de paralelismo.
        Self {
            algo: "argon2id".to_string(),
            m_cost: 19_456,
            t_cost: 2,
            p_cost: 1,
        }
    }
}

pub fn random_salt() -> Vec<u8> {
    let mut salt = vec![0u8; SALT_LEN];
    rand::rngs::OsRng.fill_bytes(&mut salt);
    salt
}

/// Deriva la clave simétrica de 32 bytes a partir de la contraseña maestra.
pub fn derive_key(
    password: &str,
    salt: &[u8],
    params: &KdfParams,
) -> Result<Zeroizing<[u8; KEY_LEN]>, String> {
    if params.algo != "argon2id" {
        return Err(format!("KDF no soportado: {}", params.algo));
    }

    let argon_params = Params::new(
        params.m_cost,
        params.t_cost,
        params.p_cost,
        Some(KEY_LEN),
    )
    .map_err(|e| format!("Parámetros Argon2 inválidos: {e}"))?;

    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon_params);

    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    argon
        .hash_password_into(password.as_bytes(), salt, key.as_mut())
        .map_err(|e| format!("Fallo al derivar la clave: {e}"))?;

    Ok(key)
}

/// Cifra `plaintext` y devuelve `(nonce, ciphertext)`.
pub fn encrypt(key: &[u8; KEY_LEN], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
    let cipher = XChaCha20Poly1305::new(key.into());

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, Payload { msg: plaintext, aad: b"passcontroller-vault-v2" })
        .map_err(|_| "Fallo al cifrar el vault".to_string())?;

    Ok((nonce_bytes.to_vec(), ciphertext))
}

/// Descifra y verifica el tag de autenticación.
pub fn decrypt(
    key: &[u8; KEY_LEN],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, String> {
    if nonce.len() != NONCE_LEN {
        return Err("Nonce con longitud inválida".to_string());
    }

    let cipher = XChaCha20Poly1305::new(key.into());
    let nonce = XNonce::from_slice(nonce);

    cipher
        .decrypt(nonce, Payload { msg: ciphertext, aad: b"passcontroller-vault-v2" })
        .map_err(|_| "Contraseña incorrecta o archivo dañado".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_ok() {
        let salt = random_salt();
        let params = KdfParams::default();
        let key = derive_key("correcto caballo batería grapa", &salt, &params).unwrap();

        let msg = br#"{"nodes":[],"items":[]}"#;
        let (nonce, ct) = encrypt(&key, msg).unwrap();
        let pt = decrypt(&key, &nonce, &ct).unwrap();

        assert_eq!(pt, msg);
    }

    #[test]
    fn wrong_password_fails() {
        let salt = random_salt();
        let params = KdfParams::default();
        let key = derive_key("clave-a", &salt, &params).unwrap();
        let (nonce, ct) = encrypt(&key, b"secreto").unwrap();

        let bad = derive_key("clave-b", &salt, &params).unwrap();
        assert!(decrypt(&bad, &nonce, &ct).is_err());
    }

    #[test]
    fn tampered_ciphertext_fails() {
        let salt = random_salt();
        let key = derive_key("clave", &salt, &KdfParams::default()).unwrap();
        let (nonce, mut ct) = encrypt(&key, b"secreto").unwrap();
        ct[0] ^= 0xff;
        assert!(decrypt(&key, &nonce, &ct).is_err());
    }
}
