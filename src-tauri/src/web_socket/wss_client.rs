use serde::{Deserialize, Serialize};
use std::{
    sync::Arc,
    time::Duration,
};
use tauri::Url;
use tauri::async_runtime::spawn;
use tokio::{
    sync::Mutex,
    net::TcpStream,
    time::sleep
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
    MaybeTlsStream,
    WebSocketStream,
};
use bytes::Bytes;
use futures_util::{
    SinkExt,
    StreamExt
};


#[derive(Clone)]
pub struct WssClientState(
    pub Arc<Mutex<WssClient>>
);


#[derive(Debug)]
pub struct WssClient {
    url: Url,
    socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    is_connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlMessage {
    pub action: String,
    pub position: String,
    pub direction: String,
    pub force: f32,
}

impl WssClient {
    pub fn new_split(mut url: String, port: u16) -> Self {
        url = WssClient::check_url(url);
        Self {
            url: Url::parse(format!("{}:{}", url, port.to_string()).as_str())
                .expect("Invalid WSS URL"),
            socket: None,
            is_connected: false,
        }
    }
    pub fn new(mut url: String) -> Self {
        url = WssClient::check_url(url);
        Self {
            url: Url::parse(url.as_str()).expect("Invalid WSS URL"),
            socket: None,
            is_connected: false,
        }
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
                self.socket = Some(ws_stream);
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

    pub async fn reconnect_with_url(&mut self, mut url: String, port: u16) -> bool {
        self.disconnect().await;

        url = WssClient::check_url(url);

        self.url = Url::parse(&format!("{}:{}", url, port)).expect("Invalid WSS URL");
        self.connect().await
    }

    pub async fn send(&mut self, msg: &ControlMessage) -> bool {
        if !self.is_connected || self.socket.is_none() {
            return false;
        }

        if let Some(socket) = &mut self.socket.take() {
            let payload = serde_json::to_string(msg).unwrap();
            if let Err(e) = socket.send(Message::Text(payload.into())).await {
                eprintln!("❌ Send failed: {}", e);
                self.is_connected = false;
                return false;
            }
            true
        } else {
            false
        }
    }

    pub async fn disconnect(&mut self) {
        if let Some(socket) = &mut self.socket.take() {
            let _ = socket.close(None).await;
        }
        self.is_connected = false;
        println!("👋 Disconnected.");
    }

    fn check_connected(&mut self) -> bool {
        if !self.is_connected || self.socket.is_none() {
            self.is_connected = false;
            false
        } else {
            true
        }
    }

    pub fn start_heartbeat(self_arc: Arc<Mutex<Self>>) {
        spawn(async move {
            loop {
                sleep(Duration::from_secs(10)).await;

                let mut client = self_arc.lock().await;

                if let Some(socket) = &mut client.socket {
                    if let Err(e) = socket.send(Message::Ping(Bytes::new())).await {
                        println!("💔 心跳失敗: {}，正在重連...", e);
                        client.is_connected = false;
                        client.disconnect().await;
                        client.connect().await;
                        continue;
                    }

                    match socket.next().await {
                        Some(Ok(Message::Pong(_))) => {
                            println!("💓 收到 Pong 回應，連線正常");
                        }
                        Some(Ok(Message::Ping(payload))) => {
                            println!("📡 收到伺服器 Ping：{:?}，自動回 Pong", payload);
                            if let Err(e) = socket.send(Message::Pong(payload)).await {
                                println!("⚠️ 回應 Pong 失敗: {}", e);
                            }
                        }
                        Some(Ok(Message::Close(frame))) => {
                            println!("🛑 伺服器主動關閉連線：{:?}", frame);
                            client.is_connected = false;
                            client.disconnect().await;
                            continue;
                        }
                        Some(Ok(other)) => {
                            println!("📥 收到其他訊息：{:?}", other);
                        }
                        Some(Err(e)) => {
                            println!("💥 讀取失敗: {e}，斷線重連");
                            client.is_connected = false;
                            client.disconnect().await;
                            client.connect().await;
                        }
                        None => {
                            println!("🔌 伺服器無回應，重連");
                            client.is_connected = false;
                            client.disconnect().await;
                            client.connect().await;
                        }
                    }


                } else {
                    println!("🔌 未連線，嘗試連線中...");
                    client.connect().await;
                }
            }
        });
    }
}
