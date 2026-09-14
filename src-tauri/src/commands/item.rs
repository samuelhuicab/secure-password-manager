use tauri::AppHandle;

use crate::{
    models::{
        field::Field,
        item::VaultItem,
        item_type::ItemType,
    },
    services::item_service::ItemService,
};

#[tauri::command]
pub fn get_items(
    app: AppHandle,
) -> Result<Vec<VaultItem>, String> {

    ItemService::get_all(&app)

}

#[tauri::command]
pub fn get_items_by_node(
    app: AppHandle,
    node_id: String,
) -> Result<Vec<VaultItem>, String> {

    ItemService::get_by_node(
        &app,
        node_id,
    )

}

#[tauri::command]
pub fn create_item(
    app: AppHandle,
    node_id: String,
    title: String,
    item_type: ItemType,
) -> Result<VaultItem, String> {

    ItemService::create(
        &app,
        node_id,
        title,
        item_type,
    )

}

#[tauri::command]
pub fn delete_item(
    app: AppHandle,
    id: String,
) -> Result<(), String> {

    ItemService::delete(
        &app,
        id,
    )

}

#[tauri::command]
pub fn update_item_title(
    app:AppHandle,
    id:String,
    title:String,
)->Result<(),String>{

    ItemService::update_title(
        &app,
        id,
        title,
    )

}

#[tauri::command]
pub fn update_item_notes(
    app:AppHandle,
    id:String,
    notes:String,
)->Result<(),String>{

    ItemService::update_notes(
        &app,
        id,
        notes,
    )

}

#[tauri::command]
pub fn toggle_item_favorite(
    app:AppHandle,
    id:String,
)->Result<(),String>{

    ItemService::toggle_favorite(
        &app,
        id,
    )

}

#[tauri::command]
pub fn update_item_field(
    app:AppHandle,
    item_id:String,
    key:String,
    value:String,
)->Result<(),String>{

    ItemService::update_field(
        &app,
        item_id,
        key,
        value,
    )

}

#[tauri::command]
pub fn add_item_field(
    app: AppHandle,
    item_id: String,
    label: String,
    hidden: bool,
) -> Result<Field, String> {
    ItemService::add_field(&app, item_id, label, hidden)
}

#[tauri::command]
pub fn remove_item_field(
    app: AppHandle,
    item_id: String,
    field_id: String,
) -> Result<(), String> {
    ItemService::remove_field(&app, item_id, field_id)
}

#[tauri::command]
pub fn rename_item_field(
    app: AppHandle,
    item_id: String,
    field_id: String,
    label: String,
) -> Result<(), String> {
    ItemService::rename_field(&app, item_id, field_id, label)
}

#[tauri::command]
pub fn move_item(
    app: AppHandle,
    item_id: String,
    node_id: String,
) -> Result<(), String> {

    ItemService::move_item(
        &app,
        item_id,
        node_id,
    )

}

#[tauri::command]
pub fn duplicate_item(
    app: AppHandle,
    id: String,
) -> Result<VaultItem, String> {

    ItemService::duplicate(
        &app,
        id,
    )

}

#[tauri::command]
pub fn get_favorites(
    app: AppHandle,
) -> Result<Vec<VaultItem>, String> {

    ItemService::favorites(
        &app,
    )

}