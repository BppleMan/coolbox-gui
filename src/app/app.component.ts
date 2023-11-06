import {Component} from "@angular/core"
import {HighlightAutoResult} from "ngx-highlightjs"

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent {
    title = "coolbox-gui"

    onHighlight(e: HighlightAutoResult) {
        console.log({
            language: e.language,
            relevance: e.relevance,
            secondBest: "{...}",
            value: "{...}",
        })
    }
}
