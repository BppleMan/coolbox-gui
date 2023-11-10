import {Injectable} from "@angular/core"

import {invoke} from "@tauri-apps/api"
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
}
