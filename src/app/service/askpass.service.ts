import {Injectable} from "@angular/core"

import {invoke} from "@tauri-apps/api"
import {listen} from "@tauri-apps/api/event"

@Injectable({
    providedIn: "root",
})
export class AskPassService {
    constructor() {
    }

    async callback_ask_pass(password: string): Promise<void> {
        await invoke("callback_ask_pass", {password: password})
    }

    async listen_ask_pass(): Promise<void> {
        await listen("ask_pass", async () => {
            await this.callback_ask_pass("bppleman")
        })
    }
}
