import { CommonModule } from "@angular/common"
import { Component, OnInit } from "@angular/core"
import { RouterOutlet } from "@angular/router"
import { TranslateService } from "@ngx-translate/core"
import { CoolCardComponent } from "./cool-card/cool-card.component"
import { CoolListComponent } from "./cool-list/cool-list.component"
import { InfoComponent } from "./info/info.component"
import { AskPassService } from "./service/askpass.service"

@Component({
    selector: "app-root",
    standalone: true,
    imports: [
        CommonModule,
        RouterOutlet,
        InfoComponent,
        CoolListComponent,
        CoolCardComponent,
    ],
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent implements OnInit {
    title = "coolbox-gui"

    constructor(
        private translateService: TranslateService,
        private ask_pass_service: AskPassService,
    ) {
        this.translateService.setDefaultLang("en-US")
    }

    ngOnInit(): void {
        this.ask_pass_service
            .listen_ask_pass()
            .then(() => {
                console.log("listen_ask_pass then")
            })
            .catch((error) => {
                console.error("listen_ask_pass catch", error)
            })
    }
}
