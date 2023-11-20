import { Injectable, NgZone } from "@angular/core"

import { MatDialog } from "@angular/material/dialog"

import { invoke } from "@tauri-apps/api"
import { listen } from "@tauri-apps/api/event"
import { PasswordDialogComponent } from "../password-dialog.component"

@Injectable({
    providedIn: "root",
})
export class AskPassService {
    constructor(
        private zone: NgZone,
        private mat_dialog: MatDialog,
    ) {}

    async callback_ask_pass(password: string): Promise<void> {
        await invoke("callback_ask_pass", { password: password })
    }

    async listen_ask_pass(): Promise<void> {
        // eslint-disable-next-line no-async-promise-executor
        return new Promise<void>(async (resolve, reject) => {
            try {
                await listen("ask_pass", async () => {
                    this.zone
                        .run(async () => {
                            const dialog_ref = this.mat_dialog.open(
                                PasswordDialogComponent,
                                {
                                    data: { title: "DIALOG.PASSWORD_TITLE" },
                                },
                            )
                            dialog_ref
                                .afterClosed()
                                .subscribe(async (password) => {
                                    await this.callback_ask_pass(
                                        password || "",
                                    )
                                })
                        })
                        .then()
                })
            } catch (error) {
                reject(error)
            }
        })
    }
}
