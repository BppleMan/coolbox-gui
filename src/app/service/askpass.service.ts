import {Injectable} from "@angular/core"

import {invoke} from "@tauri-apps/api"
import {listen} from "@tauri-apps/api/event"

@Injectable({
    providedIn: "root",
})
export class AskPassService {
    constructor() {
    }

    async callback_askpass(password: string): Promise<void> {
        await invoke("callback_askpass", {password: password})
    }

    async listen_askpass(): Promise<void> {
        await listen("ask-pass", async () => {
            await this.callback_askpass("bppleman")
        })
    }
}
