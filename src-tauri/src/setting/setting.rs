use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use dirs_next::config_dir;
use tauri::Error;
use tauri::State;

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

fn get_settings_path() -> PathBuf {
    config_dir()
        .unwrap()
        .join("bot-arena-tauri/settings.json")
}

#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<(), Error> {
    let path = get_settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(&settings)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn load_settings_from_file() -> Settings {
    fs::read_to_string(get_settings_path())
        .ok()
        .and_then(|json_str| 
            serde_json::from_str::<Settings>(&json_str).ok()
        )
        .unwrap_or_else(default_settings)
}

#[tauri::command]
pub fn load_settings(state: State<Settings>) -> Settings {
    state.inner().clone()
}

fn default_settings() -> Settings {
    Settings {
        control: Control {
            joystick_sensitivity: 0.2,
            joystick_size: 150,
        },
        connect: Connect {
            url: "http://raspberrypi".to_string(),
            port: 60922,
        },
    }
}
