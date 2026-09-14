import { invoke } from "@tauri-apps/api/core";

/* ===========================
   AUTH / CONTRASEÑA MAESTRA
=========================== */

export async function vaultStatus() {
    return await invoke("vault_status");
}

export async function createMasterPassword(password) {
    return await invoke("create_master_password", { password });
}

export async function unlockVault(password, remember) {
    return await invoke("unlock_vault", { password, remember });
}

export async function unlockWithKeychain() {
    return await invoke("unlock_with_keychain");
}

export async function lockVault() {
    return await invoke("lock_vault");
}

export async function touchActivity() {
    return await invoke("touch_activity");
}

export async function changeMasterPassword(current, next) {
    return await invoke("change_master_password", { current, new: next });
}

export async function forgetKeychain() {
    return await invoke("forget_keychain");
}

/* ===========================
   AJUSTES
=========================== */

export async function getSettings() {
    return await invoke("get_settings");
}

export async function updateSettings(settings) {
    return await invoke("update_settings", { settings });
}
