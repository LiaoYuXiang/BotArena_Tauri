import { ref } from "vue";
import { Mutex } from 'async-mutex';
import {actionControl_api} from "./tauri_api.ts";

export class JoyStickData {
    angle: number;
    direction: "up" | "down" | "left" | "right";
    force: number;

    constructor(
        angle: number,
        direction: "up" | "down" | "left" | "right",
        force: number,
    ) {
        this.angle = angle
        this.direction = direction
        this.force = force
    }
}

export class JoyStickControl {
    // 控制位置
    position: "feet" | "arm"

    // 防抖間隔
    debounceDuration: number = 0.2
    // 防抖計時 Timer
    debounceTimer: ReturnType<typeof setTimeout> | null = null;

    // 重複發送間隔
    keepActionDuration: number = 0.2;
    // 重複發送狀態(部分決定可不可以繼續發送下去)
    keepActionState: boolean = false;
    // 重複發送計時 Timer
    keepActionTimer: ReturnType<typeof setTimeout> | null = null;
    // 重複發送異步 safe-guard(指的是同一時間，確定只會有一個 keepAction 在運作)
    private keepActionMutex = new Mutex();

    // 搖桿靈敏度
    joystickThreshold = ref<number | null>(null);
    // 最後一次搖桿位置，用於防抖比對
    lastJoyStickData: JoyStickData | null = null

    webSocketState: boolean = false

    private log: boolean = false
    private alive: boolean = false

    constructor(
        position: "feet" | "arm",
        debounceDuration: number,
        keepActionDuration: number,
        joystickThreshold: number
    ) {
        this.position = position ?? "feet"
        this.debounceDuration = Math.abs(debounceDuration ?? 200)
        this.keepActionDuration = Math.abs(keepActionDuration ?? 200)
        this.joystickThreshold.value = Math.abs(joystickThreshold ?? 0.2)
    }

    private async setDebounce(joyStickData: JoyStickData) {
        if (this.log) console.log("setDebounce")
        this.clear()
        this.debounceTimer =
            setTimeout(async() => {
                // angle.value = payload.angle;
                // direction.value = payload.direction;
                // force.value = payload.force;
                // 如果資料不一樣，那就清除全部，重新計算
                // if (this.log) {
                //     console.log("runDebounce")
                //     console.log("joyStickData")
                //     console.log(joyStickData)
                //     console.log("lastJoyStickData")
                //     console.log(this.lastJoyStickData)
                //     console.log(`same: ${this.checkIsSameSignal(joyStickData, this.lastJoyStickData)}`)
                // }
                if (!(this.checkIsSameSignal(joyStickData, this.lastJoyStickData))) {
                    this.keepActionState = false
                    this.clear()
                    return
                }
                /** 最終方向 */
                // console.log(`方向:${joyStickData.force}力道:${joyStickData.direction}`)
                if (this.log) console.log("runSetKeepAction")
                this.clearKeepAction()
                this.keepActionState = true
                await this.setKeepAction(joyStickData)
            },
            this.debounceDuration
        )
    }

    private async setKeepAction(joyStickData: JoyStickData) {
        await this.keepActionMutex.runExclusive(async () => {
            if (!this.keepActionState) {
                this.clearKeepAction()
                return
            }

            if (this.log) console.log("setKeepAction")
            this.clearKeepAction()
            this.keepActionState = true;
            this.keepActionTimer = setTimeout(async () => {
                    if (!this.keepActionState) {
                        this.clearKeepAction()
                        return
                    }

                    try {
                        this.webSocketState = await actionControl_api.controlAction({
                            position: this.position,
                            direction: joyStickData.direction,
                            force: joyStickData.force,
                        });

                        if (!this.webSocketState) {
                            this.clear();
                            return;
                        }
                        this.clearKeepAction(); // 清前一次 timer
                        this.keepActionState = true;
                        if (!this.keepActionState) {
                            this.clearKeepAction()
                            return
                        }

                        if (this.alive) await this.setKeepAction(joyStickData); // 遞迴下一輪
                    } catch (e) {
                        console.error("🚨 controlAction 錯誤：", e);
                        this.clear();
                    }
                },
                this.keepActionDuration
            )
        })
    }

    /**
     * 清除防抖與重複發送的狀態
     */
    private clear() {
        this.clearDebounce()
        this.clearKeepAction()
    }
    /**
     * 清除防抖的狀態
     */
    private clearDebounce() {
        if (this.debounceTimer) {
            clearTimeout(this.debounceTimer);
            this.debounceTimer = null;
        }
    }
    /**
     * 清除重複發送的狀態
     */
    private clearKeepAction() {
        this.keepActionState = false
        this.keepActionMutex.cancel
        if (this.keepActionTimer) {
            clearTimeout(this.keepActionTimer);
            this.keepActionTimer = null;
        }
    }

    onStart() {
        this.alive = true
    }

    async onMove(joyStickData : JoyStickData) {
        // if (this.log)  console.log("onMove")
        /** 搖桿靈敏度 */
        const threshold = this.joystickThreshold.value ?? 0.2;
        /** 限制最大輸出 */
        joyStickData.force = Math.min(joyStickData.force ?? 0, 1.0);
        if (this.log) console.log(joyStickData.force)
        if (joyStickData.force < threshold) {
            this.clear()
            return;
        }
        const isSamePayload = this.checkIsSameSignal(
            this.lastJoyStickData,
            joyStickData
        );
        this.lastJoyStickData = { ...joyStickData }

        // 當沒有執行 keepAction 的時候，直接先 setDebounce
        if (!this.keepActionState) {
            await this.setDebounce(joyStickData)
            return
        }
        // 否則就是當兩次資料不相同的時候，才執行 setDebounce
        if (!isSamePayload) {
            await this.setDebounce(joyStickData)
            return
        }
    }

    async onEnd() {
        this.lastJoyStickData = null
        this.clear()
        this.webSocketState = await actionControl_api.stopAction({
            position: this.position,
        });
        this.alive = false
    };

    private checkIsSameSignal(a: JoyStickData | null, b: JoyStickData | null): boolean {
        if (!a || !b) return false
        return a.direction === b.direction && Math.abs(a.force - b.force) < 0.05;
    }
}