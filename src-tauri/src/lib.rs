use std::sync::Mutex;

mod setting;
use setting::setting::{
    load_settings_from_file,
    save_settings,
    load_settings,
    get_app_version
};

mod control_action;
use control_action::control_action::{
    robot_control_action,
    robot_stop_action,
};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(load_settings_from_file()))
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,

            get_app_version,

            robot_control_action,
            robot_stop_action,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
