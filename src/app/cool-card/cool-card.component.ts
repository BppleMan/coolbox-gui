import {CommonModule} from "@angular/common"
import {Component, Input, ViewChild} from "@angular/core"
import {MatButtonModule} from "@angular/material/button"
import {MatCheckboxModule} from "@angular/material/checkbox"
import {MatRippleModule} from "@angular/material/core"
import {MatExpansionModule, MatExpansionPanel} from "@angular/material/expansion"
import {MatIconModule} from "@angular/material/icon"
import {MatStepperModule} from "@angular/material/stepper"
import {MatDividerModule} from "@angular/material/divider"
import {Cool} from "../model/models"
import {HIGHLIGHT_OPTIONS, HighlightModule} from "ngx-highlightjs"

@Component({
    selector: "app-cool-card",
    standalone: true,
    imports: [CommonModule, MatButtonModule, MatCheckboxModule, MatExpansionModule, MatIconModule, MatStepperModule, MatRippleModule, HighlightModule, MatDividerModule],
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
    ]
})
export class CoolCardComponent {
    @ViewChild("expansionPanel") panel!: MatExpansionPanel
    @Input() cool!: Cool
    selected = false
    expanded = false
    consoleCode = 'mkdir xxx-project && cp a b'
    constructor() {
    }

    // toggle_panel(event: MouseEvent) {
    //     event.preventDefault()
    //     event.stopPropagation()
    //     this.panel.toggle()
    // }

    toggle_panel(event: MouseEvent) {
        console.log('toggle_selected')
        event.preventDefault()
        this.expanded = !this.expanded
        this.panel.toggle()
    }
}
