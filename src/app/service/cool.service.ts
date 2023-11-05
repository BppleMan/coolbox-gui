import {Injectable} from "@angular/core"

import {invoke} from "@tauri-apps/api"
import {from, map, Observable} from "rxjs"
import {Cool, ICool} from "../model/cool"
import {Task} from "../model/task"

@Injectable({
    providedIn: "root",
})
export class CoolService {
    constructor() {
    }

    cool_list(): Observable<Cool[]> {
        let promise: Promise<ICool[]> = invoke("serialize_cool_list")
        return from(promise).pipe(
            map((cool_list) => {
                return cool_list.map(c => {
                    let install_tasks: Task[] = Object.values(c.install_tasks[0])
                    let uninstall_tasks: Task[] = Object.values(c.uninstall_tasks[0])
                    let check_tasks: Task[] = Object.values(c.check_tasks[0] ?? [])
                    return new Cool(c.name, c.description, c.dependencies, install_tasks, uninstall_tasks, check_tasks)
                })
            }),
        )
    }
}
