import {CommonModule} from "@angular/common"
import {NgModule} from "@angular/core"
import {BrowserModule} from "@angular/platform-browser"
import {BrowserAnimationsModule} from "@angular/platform-browser/animations"
import {HIGHLIGHT_OPTIONS, HighlightModule} from "ngx-highlightjs"

import {AppComponent} from "./app.component"
import {CoolCardComponent} from "./cool-card/cool-card.component"
import {CoolListComponent} from "./cool-list/cool-list.component"
import {InfoComponent} from "./info/info.component"
import { MatDialogModule } from "@angular/material/dialog"

@NgModule({
    declarations: [
        AppComponent,
    ],
    imports: [
        CommonModule,
        BrowserModule,
        BrowserAnimationsModule,
        InfoComponent,
        CoolListComponent,
        CoolCardComponent,
        HighlightModule,
        MatDialogModule
    ],
    providers: [
        {
            provide: HIGHLIGHT_OPTIONS,
            useValue: {
                coreLibraryLoader: () => import("highlight.js/lib/core"),
                languages: {
                    typescript: () => import("highlight.js/lib/languages/typescript"),
                    bash: () => import("highlight.js/lib/languages/bash"),
                    // css: () => import("highlight.js/lib/languages/css"),
                    // xml: () => import("highlight.js/lib/languages/xml"),
                    // bash: () => import("highlight.js/lib/languages/bash"),
                },
            },
        },
    ],
    bootstrap: [AppComponent],
})
export class AppModule {
}
