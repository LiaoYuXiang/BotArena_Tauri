import { invoke } from "@tauri-apps/api/core";
/** 軟體控制設定物件格式 */
export interface Control {
  joystick_sensitivity: number;
  joystick_size: number;
}

/** 軟體控連線定物件格式 */
export interface Connect {
  url: string;
  port: number;
}
/** 軟體設定物件格式 */
export interface Settings {
  control: Control;
  connect: Connect;
}

/**
 * 取得設定
 * @returns Settings
 */
const getSetting = async (): Promise<Settings> => {
  return await invoke<Settings>("load_settings", {});
};
/**
 * 寫入設定
 * @param settings
 */
const setSetting = async (settings: Settings) => {
  await invoke("save_settings", { settings });
};

/**
 * settings
 * @returns 版本號
 */
const getVersion = async (): Promise<string> => {
  return await invoke<string>("get_app_version", {});
};
/** 設定相關功能 */
export const setting_api = {
  getSetting,
  setSetting,
  getVersion,
};

/** 控制行為物件格式 */
export interface ActionControl {
  position: "feet" | "arm";
  direction: "up" | "down" | "left" | "right";
  force: number;
}
/** 停止行為物件格式 */
export interface StopAction {
  position: "feet" | "arm";
}

/**
 * 控制行為（通用）
 * @param action
 * @returns 操作是否成功
 */
export const controlAction = async (
  action: ActionControl
): Promise<boolean> => {
  return await invoke<boolean>("robot_control_action_ws", {
    actionControl: action,
  });
};

/**
 * 停止行為（通用）
 * @param stop
 * @returns 操作是否成功
 */
export const stopAction = async (stop: StopAction): Promise<boolean> => {
  return await invoke<boolean>("robot_stop_action_ws", {
    stopAction: stop,
  });
};

/** 機器人行為相關功能 */
export const actionControl_api = {
  controlAction,
  stopAction,
};

/**
 * 根據設定檔重新連接 WebSocket (請在來開設定畫面，並且確定 Settings.Control 有進行更動再呼叫)
 * @returns 是否重新連接成功
 */
export const reconnectWs = async (): Promise<boolean> => {
  return await invoke<boolean>("reconnect_ws", {});
};

/**
 * 檢查是否連接上WebSocket伺服器
 * @returns 是否連接上WebSocket伺服器
 */
export const wsIsConnected = async (): Promise<boolean> => {
  return await invoke<boolean>("ws_is_connected", {});
};

/** WebSocket相關功能 */
export const webSocket_api = {
  reconnectWs,
  wsIsConnected,
};