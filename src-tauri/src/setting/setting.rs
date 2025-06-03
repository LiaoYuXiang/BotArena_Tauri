use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Error, Manager, Runtime};
use tauri::State;
use crate::web_socket::wss_client::WsClientState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Control {
    pub joystick_sensitivity: f64,
    pub joystick_size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Connect {
    pub url: String,
    pub port: u16, // u16 就是 short（0 ~ 65535）
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub control: Control,
    pub connect: Connect,
}

fn get_settings_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path().config_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("bot-arena-tauri/settings.json")
}

fn default_settings() -> Settings {
    Settings {
        control: Control {
            joystick_sensitivity: 0.2,
            joystick_size: 150,
        },
        connect: Connect {
            url: "raspberrypi".to_string(),
            port: 60922,
        },
    }
}

pub fn load_settings_from_file<R: Runtime>(app: &AppHandle<R>) -> Settings {
    let path = get_settings_path(app);
    if !path.exists() {
        return default_settings()
    }
    match fs::read_to_string(path) {
        Ok(json_str) => {
            serde_json::from_str::<Settings>(&json_str)
                .unwrap_or_else(|_e| default_settings())
        },
        Err(_e) => default_settings(),
    }
}

#[tauri::command]
pub fn save_settings<R: Runtime>(
    app: AppHandle<R>,
    settings: Settings,
    ws_state: State<WsClientState>,
    file_state: State<Mutex<Settings>>,
) -> Result<(), Error> {
    // 儲存到設定檔
    save_settings_to_file(app, settings.clone())?;
    
    // 嘗試重連 WebSocket
    let ws_clone = ws_state.inner().clone();
    let settings_clone = settings.clone();
    tauri::async_runtime::spawn(async move {
        let mut ws = ws_clone.0.lock().await;
        ws.reconnect_with_url(
            settings_clone.connect.url.clone(),
            settings_clone.connect.port.clone()
        ).await;
    });
    
    // ✅ 更新記憶體中的 state
    let mut file = file_state.lock().unwrap();
    *file = settings;
    Ok(())
}

fn save_settings_to_file<R: Runtime>(
    app: AppHandle<R>,
    settings: Settings,
) -> Result<(), Error> {
    // ✅ 寫入檔案
    let path = get_settings_path(&app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(&settings)?;
    fs::write(path, content)?;
    Ok(())
}

#[tauri::command]
pub fn load_settings(state: State<Mutex<Settings>>) -> Settings {
    state.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}