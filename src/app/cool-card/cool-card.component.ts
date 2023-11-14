import {CommonModule} from "@angular/common"
import {Component, EventEmitter, Input, OnChanges, OnInit, Output, ViewChild} from "@angular/core"
import {MatButtonModule} from "@angular/material/button"
import {MatCheckboxModule} from "@angular/material/checkbox"
import {MatRippleModule} from "@angular/material/core"
import {MatExpansionModule, MatExpansionPanel} from "@angular/material/expansion"
import {MatIconModule} from "@angular/material/icon"
import {MatStepperModule} from "@angular/material/stepper"
import {MatDividerModule} from "@angular/material/divider"
import {MatProgressBarModule} from "@angular/material/progress-bar"
import {BehaviorSubject} from "rxjs"
import {Cool, CoolListItem} from "../model/models"
import {HIGHLIGHT_OPTIONS, HighlightModule} from "ngx-highlightjs"
import {CoolService} from "../service/cool.service"

@Component({
    selector: "app-cool-card",
    standalone: true,
    imports: [CommonModule, MatButtonModule, MatCheckboxModule, MatExpansionModule, MatIconModule, MatStepperModule, MatRippleModule, HighlightModule, MatDividerModule, MatProgressBarModule],
    templateUrl: "./cool-card.component.html",
    styleUrls: ["./cool-card.component.scss"],
    providers: [
        {
            provide: HIGHLIGHT_OPTIONS,
            useValue: {
                coreLibraryLoader: () => import("highlight.js/lib/core"),
                languages: {
                    typescript: () => import("highlight.js/lib/languages/typescript"),
                    bash: () => import("highlight.js/lib/languages/bash"),
                },
            },
        },
    ],
})
export class CoolCardComponent implements OnInit {
    @ViewChild("expansionPanel") panel!: MatExpansionPanel
    @Input() cool!: Cool
    expanded = false
    consoleCode = "mkdir xxx-project && cp a b"

    constructor(private cool_service: CoolService) {
    }

    ngOnInit(): void {
        this.cool.events.subscribe((events) => {
            console.log("events", events)
        })
    }

    toggle_select(event: MouseEvent) {
        event.preventDefault()
        event.stopPropagation()
        this.cool.selected.next(!this.cool.selected.value)
    }

    toggle_panel(event: MouseEvent) {
        console.log("toggle_selected")
        event.preventDefault()
        event.stopPropagation()
        this.panel.toggle()
    }

    setOpened() {
        this.expanded = true
    }

    setClosed() {
        this.expanded = false
    }

    copy(content: string) {
        console.log("copy", content)
        navigator.clipboard.writeText(content)
        // TODO we should show toast here to indicate the copy action
    }

    install(event: MouseEvent) {
        event.stopPropagation()
        this.cool_service.install_cool([this.cool]).then()
    }
}
