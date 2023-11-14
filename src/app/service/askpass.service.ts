import {Injectable} from "@angular/core"

import {MatDialog} from "@angular/material/dialog"

import {invoke} from "@tauri-apps/api"
import {listen} from "@tauri-apps/api/event"
import {PasswordDialogComponent} from "../password-dialog.component"

@Injectable({
    providedIn: "root",
})
export class AskPassService {
    constructor(private dialog: MatDialog) {
    }

    async callback_ask_pass(password: string): Promise<void> {
        await invoke("callback_ask_pass", {password: password})
    }

    async listen_ask_pass(): Promise<void> {
        return new Promise<void>(async (resolve, reject) => {
            try {
                await listen("ask_pass", async () => {
                    const dialogRef = this.dialog.open(PasswordDialogComponent)
                    dialogRef.afterClosed().subscribe(async password => {
                        await this.callback_ask_pass(password || "")
                    })
                })
            } catch (error) {
                reject(error)
            }
        })
    }
}
