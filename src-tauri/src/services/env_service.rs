//! Conversión entre items del vault y archivos `.env`.

use tauri::AppHandle;
use uuid::Uuid;

use crate::models::field::Field;
use crate::models::item::VaultItem;
use crate::models::item_type::ItemType;
use crate::services::vault_service::VaultService;

pub struct EnvService;

impl EnvService {
    /// Genera el contenido `.env` a partir de los campos de un item.
    pub fn export(app: &AppHandle, item_id: &str) -> Result<String, String> {
        let vault = VaultService::load(app)?;
        let item = vault
            .items
            .iter()
            .find(|i| i.id == item_id)
            .ok_or("Item no encontrado")?;

        let mut out = String::new();
        for field in &item.fields {
            let key = normalize_key(&field.key, &field.label);
            out.push_str(&key);
            out.push('=');
            out.push_str(&quote_if_needed(&field.value));
            out.push('\n');
        }
        Ok(out)
    }

    /// Crea un item nuevo de tipo `Env` con un campo por variable.
    pub fn import(
        app: &AppHandle,
        node_id: &str,
        title: &str,
        content: &str,
    ) -> Result<VaultItem, String> {
        let pairs = parse(content);
        if pairs.is_empty() {
            return Err("No se encontraron variables en el archivo.".to_string());
        }

        let mut vault = VaultService::load(app)?;
        let now = chrono::Utc::now().to_rfc3339();

        let fields = pairs
            .into_iter()
            .map(|(k, v)| Field {
                id: Uuid::new_v4().to_string(),
                key: k.to_lowercase(),
                label: k.clone(),
                value: v,
                // Marca como oculto lo que parezca sensible.
                hidden: looks_secret(&k),
            })
            .collect();

        let item = VaultItem {
            id: Uuid::new_v4().to_string(),
            node_id: node_id.to_string(),
            title: title.to_string(),
            item_type: ItemType::Env,
            favorite: false,
            notes: String::new(),
            created_at: now.clone(),
            updated_at: now,
            fields,
        };

        vault.items.push(item.clone());
        VaultService::save(app, &vault)?;
        Ok(item)
    }
}

/// Parsea texto `.env` en pares clave/valor. Soporta comentarios (`#`),
/// líneas en blanco, `export KEY=...` y comillas simples/dobles.
pub fn parse(content: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let line = line.strip_prefix("export ").unwrap_or(line);

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        let key = key.trim();
        if key.is_empty() {
            continue;
        }

        let mut value = value.trim().to_string();

        // Quita comillas envolventes.
        if (value.starts_with('"') && value.ends_with('"') && value.len() >= 2)
            || (value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2)
        {
            value = value[1..value.len() - 1].to_string();
        } else if let Some(idx) = value.find(" #") {
            // Comentario al final de una línea sin comillas.
            value = value[..idx].trim().to_string();
        }

        out.push((key.to_string(), value));
    }

    out
}

fn normalize_key(key: &str, label: &str) -> String {
    let base = if key.trim().is_empty() { label } else { key };
    base.trim()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .to_uppercase()
}

fn quote_if_needed(value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }
    let needs = value.contains(|c: char| c.is_whitespace() || c == '#' || c == '"' || c == '\'');
    if needs {
        format!("\"{}\"", value.replace('"', "\\\""))
    } else {
        value.to_string()
    }
}

fn looks_secret(key: &str) -> bool {
    let k = key.to_lowercase();
    ["secret", "token", "key", "pass", "pwd", "auth", "credential", "private"]
        .iter()
        .any(|needle| k.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_common_shapes() {
        let input = r#"
# comentario
export API_URL=https://example.com
DB_PASS="p@ss word"
TOKEN='abc123'
EMPTY=
INLINE=value # nota
"#;
        let pairs = parse(input);
        assert_eq!(pairs.len(), 5);
        assert_eq!(pairs[0], ("API_URL".into(), "https://example.com".into()));
        assert_eq!(pairs[1], ("DB_PASS".into(), "p@ss word".into()));
        assert_eq!(pairs[2], ("TOKEN".into(), "abc123".into()));
        assert_eq!(pairs[3], ("EMPTY".into(), "".into()));
        assert_eq!(pairs[4], ("INLINE".into(), "value".into()));
    }

    #[test]
    fn quotes_values_with_spaces() {
        assert_eq!(quote_if_needed("hola mundo"), "\"hola mundo\"");
        assert_eq!(quote_if_needed("simple"), "simple");
    }
}
