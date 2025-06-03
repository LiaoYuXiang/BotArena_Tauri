use serde::{Deserialize, Serialize};
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::Url;
use tokio::{spawn, time::sleep};
use tungstenite::{
    connect,
    protocol::WebSocket,
    stream::MaybeTlsStream,
    Bytes,
    Message,
    Utf8Bytes
};

#[derive(Clone)]
pub struct WsClientState(
    pub Arc<Mutex<WSSClient>>
);


#[derive(Debug)]
pub struct WSSClient {
    url: Url,
    socket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    is_connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlMessage {
    pub action: String,
    pub position: String,
    pub direction: String,
    pub force: f32,
}

impl WSSClient {
    pub fn new_split(url: &str, port: u16) -> Self {
        Self {
            url: Url::parse(format!("{}:{}", url, port.to_string()).as_str()).expect("Invalid WSS URL"),
            socket: None,
            is_connected: false,
        }
    }
    pub fn new(url: &str) -> Self {
        Self {
            url: Url::parse(url).expect("Invalid WSS URL"),
            socket: None,
            is_connected: false,
        }
    }

    pub fn connect(&mut self) -> bool {
        match connect(self.url.clone().to_string()) {
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

    pub fn reconnect_with_url(&mut self, url: &str) -> bool {
        self.disconnect(); // 關閉原本的 socket
        self.url = Url::parse(url).expect("Invalid WSS URL");
        self.connect()
    }

    pub fn send(&mut self, msg: &ControlMessage) -> bool {
        if !self.check_connected() {
            return false;
        }

        if let Some(socket) = &mut self.socket {
            let payload = serde_json::to_string(msg);
            if let Err(_) = socket.send(Message::Text(Utf8Bytes::from(payload.unwrap()))) {
                self.is_connected = false;
                return false;
            }

            if let Err(_) = socket.read() {
                self.is_connected = false;
                return false;
            }

            true
        } else {
            self.is_connected = false;
            false
        }
    }

    pub fn disconnect(&mut self) -> bool {
        if let Some(socket) = &mut self.socket {
            return match socket.close(None) {
                Ok(_) => {
                    self.socket = None;
                    self.is_connected = false;
                    println!("👋 Disconnected.");
                    true
                }
                Err(_) => false,
            };
        }
        self.is_connected = false;
        false
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

                let mut client = self_arc.lock().unwrap();

                if let Some(socket) = &mut client.socket {
                    if let Err(_) = socket.send(Message::Ping(Bytes::new())) {
                        println!("💔 心跳失敗，正在重連...");
                        client.is_connected = false;
                        client.disconnect();
                        client.connect();
                        continue;
                    }

                    match socket.read() {
                        Ok(msg) => match msg {
                            Message::Pong(_) => {
                                println!("💓 收到 Pong 回應，連線正常");
                            }
                            Message::Close(frame) => {
                                println!("🛑 伺服器主動關閉連線：{:?}", frame);
                                client.is_connected = false;
                                client.disconnect();
                                continue;
                            }
                            other => {
                                println!("📥 收到非 Pong 訊息：{:?}", other);
                            }
                        },
                        Err(e) => {
                            println!("💥 讀取失敗: {e}，斷線重連");
                            client.is_connected = false;
                            client.disconnect();
                            client.connect();
                        }
                    }

                } else {
                    println!("🔌 未連線，嘗試連線中...");
                    client.connect();
                }
            }
        });
    }


}
