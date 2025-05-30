use std::collections::HashMap;
use std::sync::Mutex;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use reqwest::Client;

mod setting;
use setting::setting::{
    load_settings_from_file,
    save_settings,
    load_settings,
    get_app_version
};

mod control_action;


#[tauri::command]
async fn post_action(
    url: String,
    _angle: f64,
    direction: String,
    _force: f64
) -> bool {
    let client = Client::new();
    let mut map = HashMap::new();
    map.insert("direction", direction);
    match client.post(url)
        .json(&map)
        .send()
        .await {
        Ok(resp) => {
            resp.status().is_success()
        }
        Err(_) => {
            false
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(load_settings_from_file()))
        .invoke_handler(tauri::generate_handler![
            post_action,
            load_settings,
            save_settings,
            get_app_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
