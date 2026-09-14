use crate::services::secret_service::{GenerateOptions, GeneratedSecret, SecretService};

#[tauri::command]
pub fn generate_secret(options: GenerateOptions) -> Result<GeneratedSecret, String> {
    SecretService::generate(&options)
}
