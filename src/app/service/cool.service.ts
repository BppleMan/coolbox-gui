import {Injectable} from "@angular/core"

import {invoke} from "@tauri-apps/api"
import {Event, listen} from "@tauri-apps/api/event"
import {BehaviorSubject, Observable} from "rxjs"
import {TaskEvent} from "../model/event"
import {Cool} from "../model/models"

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
        return invoke("install_cools", {cools: cools.map((c) => c.name)}).then((result) => {
        })
        .catch((err) => {
            console.log(err)
        })
    }

    async listen_task_event(cool_map$: BehaviorSubject<Map<string, Cool>>) {
        await listen("task_event", async (event: Event<TaskEvent>) => {
            let cool = cool_map$.value.get(event.payload.cool_name)
            cool?.events?.next([...cool?.events?.value ?? [], event.payload])
        })
    }
}
