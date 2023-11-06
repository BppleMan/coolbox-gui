import {Injectable} from "@angular/core"

import {invoke} from "@tauri-apps/api"
import {from, map, Observable} from "rxjs"
import {Cool} from "../model/models"

@Injectable({
    providedIn: "root",
})
export class CoolService {
    constructor() {
    }

    cool_list(): Observable<Cool[]> {
        let promise: Promise<Cool[]> = invoke("serialize_cool_list")
        return from(promise)
    }
}
