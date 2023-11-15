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
import { TranslateHttpLoader } from '@ngx-translate/http-loader';
import { HttpClient, HttpClientModule } from '@angular/common/http';

import { TranslateModule, TranslateService, TranslateLoader } from '@ngx-translate/core';
export function HttpLoaderFactory(http: HttpClient) {
    return new TranslateHttpLoader(http);
  }
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
        MatDialogModule,
        HttpClientModule,
        TranslateModule.forRoot({
            loader: {
              provide: TranslateLoader,
              useFactory: HttpLoaderFactory,
              deps: [HttpClient]
            }
          })
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
        }
    ],
    bootstrap: [AppComponent],
})
export class AppModule {
    constructor(private translateService: TranslateService) {
        this.translateService.setDefaultLang("en-US")
    }
}
