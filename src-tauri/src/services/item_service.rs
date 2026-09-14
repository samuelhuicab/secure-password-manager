use chrono::Utc;
use tauri::AppHandle;
use uuid::Uuid;

use crate::{
    models::{
        field::Field,
        item::VaultItem,
        item_type::ItemType,
    },
    services::{
        template_service::TemplateService,
        vault_service::VaultService,
    },
};

fn slug(label: &str) -> String {
    let s: String = label
        .trim()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    if s.is_empty() { "campo".to_string() } else { s }
}

pub struct ItemService;

impl ItemService {
    pub fn get_all(
        app: &AppHandle,
    ) -> Result<Vec<VaultItem>, String> {

        Ok(VaultService::load(app)?.items)

    }

    pub fn get_by_node(
        app: &AppHandle,
        node_id: String,
    ) -> Result<Vec<VaultItem>, String> {

        let vault = VaultService::load(app)?;

        Ok(
            vault
                .items
                .into_iter()
                .filter(|i| i.node_id == node_id)
                .collect(),
        )

    }

    pub fn create(
        app: &AppHandle,
        node_id: String,
        title: String,
        item_type: ItemType,
    ) -> Result<VaultItem, String> {

        let mut vault = VaultService::load(app)?;

        let now = Utc::now().to_rfc3339();

        let item = VaultItem {

            id: Uuid::new_v4().to_string(),

            node_id,

            title,

            item_type: item_type.clone(),

            favorite: false,

            notes: String::new(),

            created_at: now.clone(),

            updated_at: now,

            fields: TemplateService::create_fields(&item_type),

        };

        vault.items.push(item.clone());

        VaultService::save(app, &vault)?;

        Ok(item)

    }

    pub fn update_title(
        app:&AppHandle,
        id:String,
        title:String,
    )->Result<(),String>{

        let mut vault=VaultService::load(app)?;

        let item=vault
            .items
            .iter_mut()
            .find(|i|i.id==id)
            .ok_or("Item no encontrado")?;

        item.title=title;

        item.updated_at=Utc::now().to_rfc3339();

        VaultService::save(app,&vault)

    }

    pub fn update_notes(
        app:&AppHandle,
        id:String,
        notes:String,
    )->Result<(),String>{

        let mut vault=VaultService::load(app)?;

        let item=vault
            .items
            .iter_mut()
            .find(|i|i.id==id)
            .ok_or("Item no encontrado")?;

        item.notes=notes;

        item.updated_at=Utc::now().to_rfc3339();

        VaultService::save(app,&vault)

    }

    pub fn toggle_favorite(
        app:&AppHandle,
        id:String,
    )->Result<(),String>{

        let mut vault=VaultService::load(app)?;

        let item=vault
            .items
            .iter_mut()
            .find(|i|i.id==id)
            .ok_or("Item no encontrado")?;

        item.favorite=!item.favorite;

        item.updated_at=Utc::now().to_rfc3339();

        VaultService::save(app,&vault)

    }

    pub fn update_field(
        app:&AppHandle,
        item_id:String,
        key:String,
        value:String,
    )->Result<(),String>{

        let mut vault=VaultService::load(app)?;

        let item=vault
            .items
            .iter_mut()
            .find(|i|i.id==item_id)
            .ok_or("Item no encontrado")?;

        let field=item
            .fields
            .iter_mut()
            .find(|f|f.key==key)
            .ok_or("Campo no encontrado")?;

        field.value=value;

        item.updated_at=Utc::now().to_rfc3339();

        VaultService::save(app,&vault)

    }

    pub fn add_field(
        app: &AppHandle,
        item_id: String,
        label: String,
        hidden: bool,
    ) -> Result<Field, String> {

        let mut vault = VaultService::load(app)?;

        let item = vault
            .items
            .iter_mut()
            .find(|i| i.id == item_id)
            .ok_or("Item no encontrado")?;

        // Clave única dentro del item (los comandos buscan por `key`).
        let base = slug(&label);
        let mut key = base.clone();
        let mut n = 2;
        while item.fields.iter().any(|f| f.key == key) {
            key = format!("{base}_{n}");
            n += 1;
        }

        let field = Field {
            id: uuid::Uuid::new_v4().to_string(),
            key,
            label,
            value: String::new(),
            hidden,
        };

        item.fields.push(field.clone());
        item.updated_at = Utc::now().to_rfc3339();

        VaultService::save(app, &vault)?;
        Ok(field)
    }

    pub fn remove_field(
        app: &AppHandle,
        item_id: String,
        field_id: String,
    ) -> Result<(), String> {

        let mut vault = VaultService::load(app)?;

        let item = vault
            .items
            .iter_mut()
            .find(|i| i.id == item_id)
            .ok_or("Item no encontrado")?;

        item.fields.retain(|f| f.id != field_id);
        item.updated_at = Utc::now().to_rfc3339();

        VaultService::save(app, &vault)
    }

    pub fn rename_field(
        app: &AppHandle,
        item_id: String,
        field_id: String,
        label: String,
    ) -> Result<(), String> {

        let mut vault = VaultService::load(app)?;

        let item = vault
            .items
            .iter_mut()
            .find(|i| i.id == item_id)
            .ok_or("Item no encontrado")?;

        let field = item
            .fields
            .iter_mut()
            .find(|f| f.id == field_id)
            .ok_or("Campo no encontrado")?;

        field.label = label;
        item.updated_at = Utc::now().to_rfc3339();

        VaultService::save(app, &vault)
    }

    pub fn delete(
        app: &AppHandle,
        id: String,
    ) -> Result<(), String> {

        let mut vault = VaultService::load(app)?;

        vault.items.retain(|i| i.id != id);

        VaultService::save(app, &vault)

    }

    pub fn move_item(
        app: &AppHandle,
        item_id: String,
        node_id: String,
    ) -> Result<(), String> {

        let mut vault = VaultService::load(app)?;

        let item = vault
            .items
            .iter_mut()
            .find(|i| i.id == item_id)
            .ok_or("Item no encontrado")?;

        item.node_id = node_id;
        item.updated_at = Utc::now().to_rfc3339();

        VaultService::save(app, &vault)

    }

    pub fn duplicate(
        app: &AppHandle,
        id: String,
    ) -> Result<VaultItem, String> {

        let mut vault = VaultService::load(app)?;

        let item = vault
            .items
            .iter()
            .find(|i| i.id == id)
            .ok_or("Item no encontrado")?
            .clone();

        let mut copy = item;

        copy.id = Uuid::new_v4().to_string();

        copy.title = format!("{} (copia)", copy.title);

        copy.created_at = Utc::now().to_rfc3339();
        copy.updated_at = copy.created_at.clone();

        for field in &mut copy.fields {
            field.id = Uuid::new_v4().to_string();
        }

        vault.items.push(copy.clone());

        VaultService::save(app, &vault)?;

        Ok(copy)

    }

    pub fn favorites(
        app: &AppHandle,
    ) -> Result<Vec<VaultItem>, String> {

        let vault = VaultService::load(app)?;

        Ok(

            vault
                .items
                .into_iter()
                .filter(|i| i.favorite)
                .collect()

        )

    }
}