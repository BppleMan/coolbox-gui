import {Component} from "@angular/core"
import {MatButtonModule} from "@angular/material/button"
import {MatIconModule} from "@angular/material/icon"
import {MatToolbarModule} from "@angular/material/toolbar"

@Component({
    selector: "app-info",
    templateUrl: "./info.component.html",
    styleUrls: ["./info.component.scss"],
    standalone: true,
    imports: [
        MatIconModule,
        MatToolbarModule,
        MatButtonModule,
    ],
})
export class InfoComponent {

}
