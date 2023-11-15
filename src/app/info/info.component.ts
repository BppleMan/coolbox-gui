import {Component} from "@angular/core"
import {MatButtonModule} from "@angular/material/button"
import {MatIconModule} from "@angular/material/icon"
import {MatToolbarModule} from "@angular/material/toolbar"
import { MatButtonToggleModule } from "@angular/material/button-toggle"
import {TranslateService} from "@ngx-translate/core"

@Component({
    selector: "app-info",
    templateUrl: "./info.component.html",
    styleUrls: ["./info.component.scss"],
    standalone: true,
    imports: [
        MatIconModule,
        MatToolbarModule,
        MatButtonModule,
        MatButtonToggleModule
    ],
})
export class InfoComponent {
    localize: string = "en-US"
    constructor(private translateService: TranslateService) {
        // translateService.setDefaultLang('en-US');
    }

    localizeChange(language: string) {
        console.log("localizeChange", language)
        this.translateService.use(language)
    }
}
