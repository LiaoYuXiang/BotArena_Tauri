// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet() -> String {
    match reqwest::get("http://127.0.0.1:5000").await {
        Ok(resp) => resp.text().await.unwrap_or_else(|_| "讀取回應失敗".to_string()),
        Err(_) => "無法連線到伺服器".to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
