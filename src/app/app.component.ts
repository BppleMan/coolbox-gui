import {Component, OnInit} from "@angular/core"
import {HighlightAutoResult} from "ngx-highlightjs"
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

    onHighlight(e: HighlightAutoResult) {
        console.log({
            language: e.language,
            relevance: e.relevance,
            secondBest: "{...}",
            value: "{...}",
        })
    }

    ngOnInit(): void {
        this.askpass_service.listen_askpass().then()
    }
}
