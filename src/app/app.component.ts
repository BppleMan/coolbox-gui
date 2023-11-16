import { CommonModule } from "@angular/common";
import { HttpClientModule } from "@angular/common/http";
import { Component, OnInit } from "@angular/core";
import { MatDialog, MatDialogModule } from "@angular/material/dialog";
import { BrowserModule } from "@angular/platform-browser";
import { BrowserAnimationsModule } from "@angular/platform-browser/animations";
import { RouterOutlet } from "@angular/router";
import { TranslateModule, TranslateService } from "@ngx-translate/core";
import { HighlightModule } from "ngx-highlightjs";
import { CoolCardComponent } from "./cool-card/cool-card.component";
import { CoolListComponent } from "./cool-list/cool-list.component";
import { InfoComponent } from "./info/info.component";
import { AskPassService } from "./service/askpass.service";

@Component({
    selector: "app-root",
    standalone: true,
    imports: [
        CommonModule,
        RouterOutlet,
        // BrowserModule,
        // BrowserAnimationsModule,
        InfoComponent,
        CoolListComponent,
        CoolCardComponent,
        HighlightModule,
        MatDialogModule,
        HttpClientModule,
        TranslateModule,
    ],
    providers: [
        // {
        //     provide: HIGHLIGHT_OPTIONS,
        //     useValue: {
        //         coreLibraryLoader: () => import("highlight.js/lib/core"),
        //         languages: {
        //             typescript: () =>
        //                 import("highlight.js/lib/languages/typescript"),
        //             bash: () => import("highlight.js/lib/languages/bash"),
        //             // css: () => import("highlight.js/lib/languages/css"),
        //             // xml: () => import("highlight.js/lib/languages/xml"),
        //             // bash: () => import("highlight.js/lib/languages/bash"),
        //         },
        //     },
        // },
    ],
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent implements OnInit {
    title = "coolbox-gui";

    constructor(
        private dialog: MatDialog,
        private ask_pass_service: AskPassService,
        private translateService: TranslateService,
    ) {
        this.translateService.setDefaultLang("en-US");
    }

    ngOnInit(): void {
        this.ask_pass_service
            .listen_ask_pass()
            .then(() => {
                console.log("listen_ask_pass then");
            })
            .catch((error) => {
                console.error("listen_ask_pass catch", error);
            });
    }
}
