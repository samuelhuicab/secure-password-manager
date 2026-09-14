//! Sesión del vault en memoria.
//!
//! Guarda la clave derivada mientras el vault está desbloqueado. Nunca se
//! persiste en disco (salvo, opt-in, en el llavero del SO desde `auth_service`).

use std::sync::Mutex;
use std::time::{Duration, Instant};

use zeroize::Zeroizing;

use crate::crypto::{KdfParams, KEY_LEN};

pub struct SessionInner {
    key: Option<Zeroizing<[u8; KEY_LEN]>>,
    pub kdf: KdfParams,
    pub salt: Vec<u8>,
    last_activity: Instant,
    auto_lock: Duration,
    locked: bool,
}

impl Default for SessionInner {
    fn default() -> Self {
        Self {
            key: None,
            kdf: KdfParams::default(),
            salt: Vec::new(),
            last_activity: Instant::now(),
            auto_lock: Duration::from_secs(300),
            locked: true,
        }
    }
}

#[derive(Default)]
pub struct VaultSession(pub Mutex<SessionInner>);

impl VaultSession {
    pub fn unlock(&self, key: Zeroizing<[u8; KEY_LEN]>, kdf: KdfParams, salt: Vec<u8>) {
        let mut inner = self.0.lock().unwrap();
        inner.key = Some(key);
        inner.kdf = kdf;
        inner.salt = salt;
        inner.last_activity = Instant::now();
        inner.locked = false;
    }

    pub fn lock(&self) {
        let mut inner = self.0.lock().unwrap();
        inner.key = None; // Zeroizing borra el material al soltarlo.
        inner.locked = true;
    }

    pub fn set_auto_lock(&self, seconds: u64) {
        let mut inner = self.0.lock().unwrap();
        inner.auto_lock = Duration::from_secs(seconds);
    }

    pub fn is_initialized(&self) -> bool {
        !self.0.lock().unwrap().salt.is_empty()
    }

    /// `true` si está bloqueado (por acción manual o por caducidad de inactividad).
    pub fn is_locked(&self) -> bool {
        let mut inner = self.0.lock().unwrap();
        if inner.locked {
            return true;
        }
        if !inner.auto_lock.is_zero() && inner.last_activity.elapsed() >= inner.auto_lock {
            inner.locked = true;
            inner.key = None;
            return true;
        }
        false
    }

    /// Registra actividad para reiniciar el contador de inactividad.
    pub fn touch(&self) {
        self.0.lock().unwrap().last_activity = Instant::now();
    }

    /// Devuelve una copia de la clave si el vault está desbloqueado, o el
    /// centinela `"VAULT_LOCKED"` que el frontend sabe interpretar.
    pub fn require_key(&self) -> Result<Zeroizing<[u8; KEY_LEN]>, String> {
        if self.is_locked() {
            return Err("VAULT_LOCKED".to_string());
        }
        let mut inner = self.0.lock().unwrap();
        inner.last_activity = Instant::now();
        inner
            .key
            .as_ref()
            .map(|k| Zeroizing::new(**k))
            .ok_or_else(|| "VAULT_LOCKED".to_string())
    }
}
