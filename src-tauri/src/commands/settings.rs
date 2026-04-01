use serde::Serialize;
use tauri::State;
use warpinator_lib::config::user::UserConfig;

#[derive(Serialize, Clone)]
pub struct ThemeSettings {
    pub theme: String,
}

#[derive(Serialize, Clone)]
pub struct Configuration {
    pub group_code: String,
    pub hostname: String,
    pub username: String,
    pub display_name: String,
}

#[tauri::command]
pub fn get_theme_settings(settings: State<'_, ThemeSettings>) -> ThemeSettings {
    settings.inner().clone()
}

#[tauri::command]
pub async fn get_user_config(config: State<'_, UserConfig>) -> Result<Configuration, String> {
    Ok(Configuration {
        group_code: config.group_code.clone(),
        hostname: config.hostname.clone(),
        username: config.username.clone(),
        display_name: config.display_name.read().await.clone(),
    })
}
