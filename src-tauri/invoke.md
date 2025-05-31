# TS 調用 Rust 方式

## 設定
### 前置作業
```ts
export interface Control {
    joystick_sensitivity: number
    joystick_size: number // 對應 Rust 的 u32
}

export interface Connect {
    url: string
    port: number // 對應 Rust 的 u16（0~65535 的整數）
}

export interface Settings {
    control: Control
    connect: Connect
}
```

### 取得設定
```ts
import { invoke } from '@tauri-apps/api/tauri'

async function call() {
    const result = await invoke<Settings>('load_settings', {})
    console.log(result) // Settings 物件
}
```

### 寫入設定
```ts
import { invoke } from '@tauri-apps/api/tauri'
async function call() {
    await invoke('save_settings', {
        settings: Settings = {
            control: {
                joystick_sensitivity: 0.2,
                joystick_size: 150,
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
import { invoke } from '@tauri-apps/api/tauri'
async function call() {
    let result = await invoke<string>('get_app_version', {})
    console.log(result) // 0.1.0
}
```

---

## 控制機器人
### 前置
```ts
export interface ActionControl {
    position: string,
    direction: string,
    force: number,
}
export interface StopAction {
    position: string,
}
```

### 控制腿部
```ts
import { invoke } from '@tauri-apps/api/tauri'
async function call() {
    await invoke<boolean>('robot_control_action', {
        action_control: ActionControl = {
            position: 'feet',
            direction: 'up',
            force: 1.3,
        }
    })
}
```
### 控制手部
```ts
import { invoke } from '@tauri-apps/api/tauri'
async function call() {
    await invoke<boolean>('robot_control_action', {
        action_control: ActionControl = {
            position: 'arm',
            direction: 'down',
            force: 3.4,
        }
    })
}
```
### 停止腿部動作(回覆站立姿態)
```ts
import { invoke } from '@tauri-apps/api/tauri'
async function call() {
    await invoke<boolean>('robit_stop_action', {
        stop_action: StopAction = {
            position: 'feet',
        }
    })
}
```
### 停止手部動作(我覺得不需要)
```ts
import { invoke } from '@tauri-apps/api/tauri'
async function call() {
    await invoke<boolean>('robit_stop_action', {
        stop_action: StopAction = {
            position: 'arm',
        }
    })
}
```
---
