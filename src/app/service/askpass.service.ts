import {Injectable, NgZone} from "@angular/core"

import {MatDialog} from "@angular/material/dialog"

import {invoke} from "@tauri-apps/api"
import {listen} from "@tauri-apps/api/event"
import {PasswordDialogComponent} from "../password-dialog.component"

@Injectable({
    providedIn: "root",
})
export class AskPassService {
    constructor(private dialog: MatDialog, private zone: NgZone) {
    }

    async callback_ask_pass(password: string): Promise<void> {
        await invoke("callback_ask_pass", {password: password})
    }

    async listen_ask_pass(): Promise<void> {
        // eslint-disable-next-line no-async-promise-executor
        return new Promise<void>( async (resolve, reject) => {
            try {
                await listen("ask_pass", async () => {
                    this.zone.run(async () => {
                        const dialogRef = this.dialog.open(PasswordDialogComponent, {
                            data: {title: "DIALOG.PASSWORD_TITLE"},
                        })
                        dialogRef.afterClosed().subscribe(async password => {
                            await this.callback_ask_pass(password || "")
                        })
                    })
                    
                })
            } catch (error) {
                reject(error)
            }
        })
    }
}
