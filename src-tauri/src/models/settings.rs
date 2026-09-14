//! Preferencias no sensibles de la app. Se guardan sin cifrar en `settings.json`
//! porque no contienen secretos: solo controlan el comportamiento de bloqueo.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Minutos de inactividad antes del auto-bloqueo. `0` desactiva el auto-bloqueo.
    pub auto_lock_seconds: u64,
    /// Si el usuario pidió recordar la clave en el llavero del SO.
    pub remember_device: bool,
    /// Segundos tras los que se limpia el portapapeles al copiar un secreto.
    pub clipboard_clear_seconds: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_lock_seconds: 300,
            remember_device: false,
            clipboard_clear_seconds: 20,
        }
    }
}
