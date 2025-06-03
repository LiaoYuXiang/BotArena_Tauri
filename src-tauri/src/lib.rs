use std::sync::Arc;
use tokio::sync::Mutex;
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
use web_socket::ws_client::{
    WsClient,
    WsClientState,
    robot_control_action_ws,
    robot_stop_action_ws,
    reconnect_ws,
    ws_is_connected
};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let settings = load_settings_from_file(app.handle());
            let settings_clone = settings.clone();
            app.manage(std::sync::Mutex::new(settings));

            let ws = Arc::new(Mutex::new(WsClient::new_split(
                settings_clone.connect.url,
                settings_clone.connect.port
            )));

            let ws_for_spawn = ws.clone();
            let ws_for_heartbeat = ws.clone();
            tauri::async_runtime::spawn(async move {
                let mut client = ws_for_spawn.lock().await;
                client.connect().await;
            });
            WsClient::start_heartbeat(ws_for_heartbeat.clone());

            // 註冊為全域狀態
            app.manage(WsClientState(ws));

            Ok(())
        })
        .on_window_event(move |window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    window.hide().unwrap();
                    println!("🧹 清理中...");
                    api.prevent_close();

                    // 取出真正的 Arc<Mutex<WsClient>>，生命期已經是 'static
                    let ws_arc = {
                        let app_handle = window.app_handle();
                        app_handle.state::<WsClientState>().0.clone()
                    };

                    tauri::async_runtime::spawn(async move {
                        let mut ws = ws_arc.lock().await;
                        ws.disconnect().await;
                        std::process::exit(0);
                    });
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,

            get_app_version,

            robot_control_action,
            robot_stop_action,

            robot_control_action_ws,
            robot_stop_action_ws,
            reconnect_ws,
            ws_is_connected,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
