use std::sync::Mutex;
use reqwest::Client;
use tauri::{Error, State};
use serde::{Deserialize, Serialize};

use crate::setting::setting::{
    Settings,
    load_settings,
};

pub trait Action {}

#[derive(Debug, Serialize)]
pub struct ActionControl {
    pub position: String,
    pub direction: String,
    pub force: f64,
}
impl Action for ActionControl {}

#[derive(Debug, Serialize)]
pub struct StopAction {
    pub position: String,
}
impl Action for StopAction {}

#[derive(Debug, Deserialize)]
pub struct ReturnStatus {
    pub success: bool,
    pub status: String,
}


#[tauri::command]
pub async fn robot_control_action(
    position: String,
    _angle: f64,
    direction: String,
    force: f64,
    state: State<'_, Mutex<Settings>>,
) -> Result<bool, Error> {
    let setting = load_settings(state);
    let url =
        setting.connect.url + ":" + setting.connect.port.to_string().as_str() +
            "/botarena/api/v1/control/group";
    let action_control = ActionControl {
        position,
        direction,
        force,
    };


    match post_action(url, action_control).await {
        Ok(b) => Ok(b),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
pub async fn robit_stop_action(
    position: String,
    state: State<'_, Mutex<Settings>>,
) -> Result<bool, Error> {
    let setting = load_settings(state);
    let url =
        setting.connect.url + ":" + setting.connect.port.to_string().as_str() +
            "/botarena/api/v1/control/stop";
    let stop_action = StopAction {
        position,
    };

    match post_action(url, stop_action).await {
        Ok(b) => Ok(b),
        Err(_) => Ok(false),
    }
}

async fn post_action<T>(
    url: String,
    action: T,
) -> Result<bool, Error> where T: Serialize + Action, {
    match Client::new()
        .post(&url)
        .json(&action)
        .send()
        .await {
        Ok(resp) => {
            match resp.json::<ReturnStatus>().await {
                Ok(return_status) => Ok(return_status.success),
                Err(_) => Ok(false),
            }
        }
        Err(_) => Ok(false),
    }
}