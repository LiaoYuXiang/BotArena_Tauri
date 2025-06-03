use std::sync::{Arc, Mutex};
use tauri::Manager;

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

mod web_socket;
use web_socket::wss_client::{
    WSSClient,
    WsClientState
};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let settings = load_settings_from_file(app.handle());
            app.manage(Mutex::new(settings.clone()));

            let ws = Arc::new(Mutex::new(WSSClient::new_split(
                &settings.connect.url.clone(),
                settings.connect.port.clone()
            )));
            {
                let mut client = ws.lock().unwrap();
                client.connect();
            }
            WSSClient::start_heartbeat(ws.clone());

            // 註冊為全域狀態
            app.manage(WsClientState(ws));
            Ok(())
        })
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
