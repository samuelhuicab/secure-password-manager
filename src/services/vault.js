import { invoke } from "@tauri-apps/api/core";

/* ===========================
   MANEJO DE BLOQUEO
   Cualquier comando puede devolver "VAULT_LOCKED" si la sesión caducó por
   inactividad. `call()` lo intercepta y avisa a AuthContext para mostrar la
   pantalla de bloqueo.
=========================== */

let onLocked = null;

export function setLockHandler(fn) {
    onLocked = fn;
}

async function call(cmd, args) {
    try {
        return await invoke(cmd, args);
    } catch (err) {
        if (err === "VAULT_LOCKED" || err?.message === "VAULT_LOCKED") {
            if (onLocked) onLocked();
        }
        throw err;
    }
}

/* ===========================
   NODES
=========================== */

export async function getNodes() {
    return await call("get_nodes");
}

export async function createNode(name, parentId, nodeType) {
    return await call("create_node", { name, parentId, nodeType });
}

export async function renameNode(id, name) {
    return await call("rename_node", { id, name });
}

export async function deleteNode(id) {
    return await call("delete_node", { id });
}

export async function toggleNode(id) {
    return await call("toggle_node", { id });
}

export async function moveNode(id, parentId) {
    return await call("move_node", { id, parentId });
}

/* ===========================
   ITEMS
=========================== */

export async function getItemsByNode(nodeId) {
    return await call("get_items_by_node", { nodeId });
}

export async function createItem(nodeId, title, itemType) {
    return await call("create_item", { nodeId, title, itemType });
}

export async function deleteItem(id) {
    return await call("delete_item", { id });
}

export async function duplicateItem(id) {
    return await call("duplicate_item", { id });
}

export async function updateItemTitle(id, title) {
    return await call("update_item_title", { id, title });
}

export async function updateItemNotes(id, notes) {
    return await call("update_item_notes", { id, notes });
}

export async function updateItemField(itemId, key, value) {
    return await call("update_item_field", { itemId, key, value });
}

export async function toggleFavorite(id) {
    return await call("toggle_item_favorite", { id });
}

export async function getFavorites() {
    return await call("get_favorites");
}

/* ===========================
   SEARCH
=========================== */

export async function search(query) {
    return await call("search_items", { query });
}

export async function openSsh(ip, user) {
    return await call("open_ssh", { ip, user });
}
