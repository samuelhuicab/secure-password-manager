import { invoke } from "@tauri-apps/api/core";

/* ===========================
   GENERADOR DE SECRETOS
=========================== */

export async function generateSecret(options) {
    return await invoke("generate_secret", { options });
}

/* ===========================
   .ENV
=========================== */

export async function exportEnv(itemId, path) {
    return await invoke("export_env", { itemId, path: path ?? null });
}

export async function importEnv(nodeId, title, { content = null, path = null }) {
    return await invoke("import_env", { nodeId, title, content, path });
}

/* ===========================
   PORTAPAPELES (con auto-borrado)
=========================== */

export async function copySecret(value, clearAfter) {
    return await invoke("copy_secret", { value, clearAfter });
}

export async function copyAsExport(key, value, clearAfter) {
    return await invoke("copy_as_export", { key, value, clearAfter });
}

export async function copyConnectionString(itemId, clearAfter) {
    return await invoke("copy_connection_string", { itemId, clearAfter });
}

/* ===========================
   CAMPOS A MEDIDA
=========================== */

export async function addItemField(itemId, label, hidden) {
    return await invoke("add_item_field", { itemId, label, hidden });
}

export async function removeItemField(itemId, fieldId) {
    return await invoke("remove_item_field", { itemId, fieldId });
}

export async function renameItemField(itemId, fieldId, label) {
    return await invoke("rename_item_field", { itemId, fieldId, label });
}

/* ===========================
   BACKUPS CIFRADOS
=========================== */

export async function exportBackup(path, password) {
    return await invoke("export_backup", { path, password });
}

export async function importBackup(path, password) {
    return await invoke("import_backup", { path, password });
}
