# TS 調用 Rust 方式

## 前置作業
```ts
import { invoke } from '@tauri-apps/api/core'
```

## 設定
### 前置作業
```ts
export interface Control {
    joystick_size: number // 150
    joystick_sensitivity: number // 0.2
    joystick_send_interval: number // 0.2 (0 ~ 1)
    joystick_debounce_interval: number // 0.2 (0 ~ 1)
}

export interface Connect {
    url: string // http://raspberrypi
    port: number // 60922
}

export interface Settings {
    control: Control
    connect: Connect
}
```

### 取得設定
```ts
async function call() {
    const result = await invoke<Settings>('load_settings', {})
    console.log(result) // Settings 物件
}
```

### 寫入設定
```ts
async function call() {
    await invoke('save_settings', {
        settings: Settings = {
            control: {
                joystick_size: 150,
                joystick_sensitivity: 0.2,
                joystick_send_interval: 0.2,
                joystick_debounce_interval: 0.2,
            },
            connect: {
                url: 'http://raspberrypi',
                port: 60922,
            },
        }
    })
}
```

---

## 取得版本
```ts
async function call() {
    let result = await invoke<string>('get_app_version', {})
    console.log(result) // 0.1.0
}
```

---

## 控制機器人(ws)
### 前置
```ts
export interface ActionControl {
    position: "arm" | "feet",
    direction: "up" | "down" | "left" | "right",
    force: number, // 0 ~ 1
}
export interface StopAction {
    position: "arm" | "feet",
}
```

### 控制腿部
```ts
async function call() {
    await invoke<boolean>('robot_control_action_ws', {
        actionControl: ActionControl = {
            position: 'feet',
            direction: 'up',
            force: 0.2,
        }
    })
}
```
### 控制手部
```ts
async function call() {
    await invoke<boolean>('robot_control_action_ws', {
        actionControl: ActionControl = {
            position: 'arm',
            direction: 'down',
            force: 0.4,
        }
    })
}
```
### 停止腿部動作(回覆站立姿態)
```ts
async function call() {
    await invoke<boolean>('robot_stop_action_ws', {
        stopAction: StopAction = {
            position: 'feet',
        }
    })
}
```
### 停止手部動作(我覺得不需要)
```ts
async function call() {
    await invoke<boolean>('robot_stop_action_ws', {
        stopAction: StopAction = {
            position: 'arm',
        }
    })
}
```
---

## WebSocket 操作

### 更新Url(請在來開設定畫面，並且確定 Settings.Control 有進行更動再呼叫)
```ts
async function call() {
    await invoke<boolean>('reconnect_ws', {})
}
```

### 檢查是否連線
```ts
async function call() {
    await invoke<boolean>('ws_is_connected', {})
}
```

### 檢查是否可以操作動作
```ts
async function call() {
    await invoke<boolean>('robot_check_action_can_use_ws', {
        actionControl: ActionControl = {
            position: 'arm',
            direction: '', // 隨意
            force: 0.0, // 隨意
        }
    })
}
```