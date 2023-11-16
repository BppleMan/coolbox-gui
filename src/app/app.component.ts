import {Component, OnInit} from "@angular/core"
import {AskPassService} from "./service/askpass.service"

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent implements OnInit {
    title = "coolbox-gui"

    constructor(private askpass_service: AskPassService) {
    }

    ngOnInit(): void {
        this.askpass_service.listen_ask_pass().then(() => {
            console.log("listen_ask_pass then")
        }).catch(error => {
            console.error("listen_ask_pass catch", error)
        })
    }
}
