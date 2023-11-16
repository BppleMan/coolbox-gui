import {Injectable} from "@angular/core"

import {invoke} from "@tauri-apps/api"
import {Event, listen} from "@tauri-apps/api/event"
import {BehaviorSubject} from "rxjs"
import {TaskEvent} from "../model/event"
import {Cool, CoolState} from "../model/models"

@Injectable({
    providedIn: "root",
})
export class CoolService {
    constructor() {
    }

    async cool_list(): Promise<Cool[]> {
        return invoke("serialize_cool_list")
    }

    async install_cool(cools: Cool[]): Promise<void> {
        return invoke("install_cools", {cools: cools.map((c) => c.name)}).then(() => {
        })
            .catch((err) => {
                console.log(err)
            })
    }

    async uninstall_cool(cools: Cool[]): Promise<void> {
        return invoke("uninstall_cools", {cools: cools.map((c) => c.name)}).then(() => {
        })
            .catch((err) => {
                console.log(err)
            })
    }

    async listen_task_event(cool_map$: BehaviorSubject<Map<string, Cool>>) {
        await listen("task_event", async (event: Event<TaskEvent>) => {
            console.log("task_event", event)
            const cool = cool_map$.value.get(event.payload.cool_name)
            cool?.events?.next([...(cool?.events?.value ?? []), event.payload])
        })
    }

    async check_cool(names: string[]): Promise<CoolState[]> {
        return invoke("check_cools", {cools: names})
    }
}

