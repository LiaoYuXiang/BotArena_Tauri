use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};
use tauri::{Error, State, Url};
use tauri::async_runtime::spawn;
use tokio::{
    net::TcpStream,
    sync::{Mutex, oneshot},
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
    MaybeTlsStream,
    WebSocketStream,
};
use futures_util::{
    SinkExt,
    StreamExt,
    stream::SplitStream,
    stream::SplitSink,
};
use crate::setting::setting::Settings;

pub trait Action {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionControl {
    pub position: String,
    pub direction: String,
    pub force: f64,
}
impl Action for ActionControl {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopAction {
    pub position: String,
}
impl Action for StopAction {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnStatus {
    pub success: bool,
    pub status: String,
}

#[derive(Clone)]
pub struct WsClientState(pub Arc<Mutex<WsClient>>);

#[derive(Debug)]
pub struct WsClient {
    url: Url,
    write: Option<Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>>,
    read: Option<Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>>,
    is_connected: bool,
    pending_responses: Arc<Mutex<HashMap<String, oneshot::Sender<String>>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlMessage {
    pub request_id: String,
    pub action: String,
    pub position: String,
    pub direction: String,
    pub force: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlResponse {
    pub request_id: String,
    pub success: bool,
    pub status: String,
}

fn str_to_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" => Some(true),
        "false" | "0" | "no" => Some(false),
        _ => None, // 無法轉換
    }
}

#[tauri::command]
pub async fn robot_check_action_can_use_ws(
    action_control: ActionControl,
    ws_state: State<'_, WsClientState>,
) -> Result<bool, Error> {
    let client = ws_state.0.lock().await;
    let msg = ControlMessage {
        request_id: uuid::Uuid::new_v4().to_string(),
        action: "checkControl".to_string(),
        position: action_control.position,
        direction: action_control.direction,
        force: action_control.force,
    };

    let control_response = client.send_and_wait_response(&msg).await;
    if let None = control_response {
        return Ok(false);
    }
    let can_use = str_to_bool(&*control_response.unwrap().status).unwrap_or_else(|| false);
    Ok(can_use)
}

#[tauri::command]
pub async fn robot_control_action_ws(
    action_control: ActionControl,
    ws_state: State<'_, WsClientState>,
) -> Result<bool, Error> {
    let client = ws_state.0.lock().await;
    let msg = ControlMessage {
        request_id: uuid::Uuid::new_v4().to_string(),
        action: "control".to_string(),
        position: action_control.position,
        direction: action_control.direction,
        force: action_control.force,
    };

    let control_response = client.send_and_wait_response(&msg).await;
    if let None = control_response {
        return Ok(false);
    }
    Ok(control_response.unwrap().success)
}

#[tauri::command]
pub async fn robot_stop_action_ws(
    stop_action: StopAction,
    ws_state: State<'_, WsClientState>,
) -> Result<bool, Error> {
    let client = ws_state.0.lock().await;
    let msg = ControlMessage {
        request_id: uuid::Uuid::new_v4().to_string(),
        action: "stop".to_string(),
        position: stop_action.position,
        direction: "".to_string(),
        force: 0.0,
    };

    let control_response = client.send_and_wait_response(&msg).await;
    if let None = control_response {
        return Ok(false);
    }
    Ok(control_response.unwrap().success)
}

#[tauri::command]
pub async fn reconnect_ws(
    settings_state: State<'_, std::sync::Mutex<Settings>>,
    ws_state: State<'_, WsClientState>,
) -> Result<bool, Error> {
    let settings = settings_state.lock().unwrap().clone();
    let ws_clone = ws_state.inner().clone();
    spawn(async move {
        let mut ws = ws_clone.0.lock().await;
        ws.disconnect().await;
        let url = WsClient::check_url(settings.connect.url);
        ws.url = Url::parse(&format!("{}:{}", url, settings.connect.port)).unwrap();
        ws.connect().await;
    });
    Ok(true)
}


#[tauri::command]
pub async fn ws_is_connected(ws_state: State<'_, WsClientState>) -> Result<bool, Error> {
    Ok(ws_state.0.lock().await.is_connected)
}


impl WsClient {
    pub fn new_split(mut url: String, port: u16) -> Self {
        url = WsClient::check_url(url);
        let client = Self {
            url: Url::parse(&format!("{}:{}", url, port))
                .expect("Invalid WSS URL"),
            write: None,
            read: None,
            is_connected: false,
            pending_responses: Arc::new(Mutex::new(HashMap::new())),
        };
        client
    }

    fn check_url(url: String) -> String {
        if url.starts_with("http://") {
            url.replacen("http://", "ws://", 1)
        } else if url.starts_with("https://") {
            url.replacen("https://", "ws://", 1)
        } else if url.starts_with("wss://") {
            url.replacen("wss://", "ws://", 1)
        } else {
            "ws://".to_string() + url.as_str()
        }
    }

    pub async fn connect(&mut self) -> bool {
        match connect_async(self.url.as_str()).await {
            Ok((ws_stream, _)) => {
                let (write, read) = ws_stream.split();
                self.write = Some(Arc::new(Mutex::new(write)));
                self.read = Some(Arc::new(Mutex::new(read)));
                self.is_connected = true;
                println!("✅ Connected to {}", self.url);
                true

            }
            Err(e) => {
                eprintln!("❌ Failed to connect to {}: {}", self.url, e);
                false
            }
        }
    }


    pub async fn send_and_wait_response(&self, msg: &ControlMessage) -> Option<ControlResponse> {
        if !self.is_connected {
            return None;
        }

        let payload = serde_json::to_string(msg).unwrap();
        let (tx, rx) = oneshot::channel();
        self.pending_responses.lock().await.insert(msg.request_id.clone(), tx);

        if let Some(write_arc) = &self.write {
            let mut write = write_arc.lock().await;
            if let Err(e) = write.send(Message::Text(payload.into())).await {
                eprintln!("❌ Send failed: {}", e);
                return None;
            }
        }

        match tokio::time::timeout(Duration::from_secs(5), rx).await {
            Ok(Ok(response)) => {
                serde_json::from_str::<ControlResponse>(&response).ok()
            },
            _ => None,
        }
    }

    pub async fn spawn_recv_loop(self_arc: Arc<Mutex<Self>>) {
        tokio::spawn(async move {
            loop {
                {
                    let (read_arc_opt, pending_opt, connected): (Option<_>, Option<_>, bool) = {
                        let client = self_arc.lock().await;
                        (
                            client.read.clone(),
                            Some(client.pending_responses.clone()),
                            client.is_connected
                        )
                    };

                    if !connected {
                        println!("🔄 自動重新連線中...");
                        let mut client = self_arc.lock().await;
                        let _ = client.connect().await;
                        continue;
                    }

                    if let (Some(read_arc), Some(pending)) = (read_arc_opt, pending_opt) {
                        let mut reader = read_arc.lock().await;
                        while let Some(msg) = reader.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                                        if let Some(req_id) = value["request_id"].as_str() {
                                            if let Some(tx) = pending.lock().await.remove(req_id) {
                                                let _ = tx.send(text.to_string());
                                                continue;
                                            }
                                        }
                                    }
                                    println!("📨 一般訊息：{}", text);
                                }
                                Ok(Message::Close(_)) | Err(_) => {
                                    println!("🔌 WebSocket 關閉或錯誤");
                                    let mut client = self_arc.lock().await;
                                    client.is_connected = false;
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }

                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        });
    }

    pub async fn disconnect(&mut self) {
        if let Some(write_arc) = &self.write {
            let mut write = write_arc.lock().await;
            let _ = write.send(Message::Close(None)).await;
        }
        self.is_connected = false;
        println!("👋 Disconnected.");
    }
}
