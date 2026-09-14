mod commands;
mod crypto;
mod models;
mod services;
mod session;
mod storage;
mod wordlist;

use std::process::Command;

use session::VaultSession;

/// Solo se permiten caracteres válidos en hostnames/IPs y nombres de usuario,
/// para que `ip`/`user` no puedan inyectar comandos en el shell.
fn is_safe_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 255
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
}

#[tauri::command]
fn open_ssh(ip: String, user: String) -> Result<(), String> {
    if !is_safe_token(&ip) || !is_safe_token(&user) {
        return Err("Host o usuario con caracteres no permitidos.".into());
    }

    let target = format!("{user}@{ip}");

    if cfg!(target_os = "windows") {
        // Sin shell: se lanza ssh directamente en una ventana nueva de PowerShell.
        Command::new("cmd")
            .args(["/C", "start", "powershell", "-NoExit", "-Command", "ssh", &target])
            .spawn()
            .map_err(|e| e.to_string())?;
    } else if cfg!(target_os = "macos") {
        Command::new("osascript")
            .args([
                "-e",
                &format!("tell application \"Terminal\" to do script \"ssh {target}\""),
            ])
            .spawn()
            .map_err(|e| e.to_string())?;
    } else {
        if Command::new("gnome-terminal")
            .args(["--", "ssh", &target])
            .spawn()
            .is_ok()
        {
            return Ok(());
        }
        if Command::new("konsole").args(["-e", "ssh", &target]).spawn().is_ok() {
            return Ok(());
        }
        Command::new("xterm")
            .args(["-hold", "-e", "ssh", &target])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(VaultSession::default())
        .invoke_handler(tauri::generate_handler![
            commands::auth::vault_status,
            commands::auth::create_master_password,
            commands::auth::unlock_vault,
            commands::auth::unlock_with_keychain,
            commands::auth::lock_vault,
            commands::auth::touch_activity,
            commands::auth::change_master_password,
            commands::auth::forget_keychain,

            commands::settings::get_settings,
            commands::settings::update_settings,

            commands::vault::load_vault,
            commands::vault::save_vault,
            commands::vault::export_backup,
            commands::vault::import_backup,

            commands::secret::generate_secret,

            commands::env::export_env,
            commands::env::import_env,

            commands::clipboard::copy_secret,
            commands::clipboard::copy_as_export,
            commands::clipboard::copy_connection_string,

            commands::node::get_nodes,
            commands::node::create_node,
            commands::node::rename_node,
            commands::node::toggle_node,
            commands::node::delete_node,
            commands::node::move_node,

            commands::item::get_items,
            commands::item::get_items_by_node,
            commands::item::create_item,
            commands::item::delete_item,
            commands::item::update_item_title,
            commands::item::update_item_notes,
            commands::item::toggle_item_favorite,
            commands::item::update_item_field,
            commands::item::add_item_field,
            commands::item::remove_item_field,
            commands::item::rename_item_field,
            commands::item::move_item,
            commands::item::duplicate_item,
            commands::item::get_favorites,

            commands::search::search_items,

            open_ssh
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
