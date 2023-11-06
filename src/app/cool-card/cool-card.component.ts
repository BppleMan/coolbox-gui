import {CommonModule} from "@angular/common"
import {Component, Input, ViewChild} from "@angular/core"
import {MatButtonModule} from "@angular/material/button"
import {MatCheckboxModule} from "@angular/material/checkbox"
import {MatRippleModule} from "@angular/material/core"
import {MatExpansionModule, MatExpansionPanel} from "@angular/material/expansion"
import {MatIconModule} from "@angular/material/icon"
import {MatStepperModule} from "@angular/material/stepper"
import {Cool} from "../model/models"

@Component({
    selector: "app-cool-card",
    standalone: true,
    imports: [CommonModule, MatButtonModule, MatCheckboxModule, MatExpansionModule, MatIconModule, MatStepperModule, MatRippleModule],
    templateUrl: "./cool-card.component.html",
    styleUrls: ["./cool-card.component.scss"],
})
export class CoolCardComponent {
    @ViewChild("expansionPanel") panel!: MatExpansionPanel
    @Input() cool!: Cool
    selected = false


    constructor() {
    }

    toggle_panel(event: MouseEvent) {
        event.stopPropagation()
        this.panel.toggle()
    }

    toggle_selected(event: MouseEvent) {
        event.stopPropagation()
        this.selected = !this.selected
    }
}
