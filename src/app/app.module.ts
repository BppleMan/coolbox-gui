import {CdkVirtualScrollViewport} from "@angular/cdk/scrolling"
import {NgModule} from "@angular/core"
import {BrowserModule} from "@angular/platform-browser"
import {BrowserAnimationsModule} from "@angular/platform-browser/animations"

import {AppComponent} from "./app.component"
import {CoolCardComponent} from "./cool-card/cool-card.component"
import {CoolListComponent} from "./cool-list/cool-list.component"
import {InfoComponent} from "./info/info.component"

@NgModule({
    declarations: [
        AppComponent,
    ],
    imports: [
        BrowserModule,
        BrowserAnimationsModule,
        InfoComponent,
        CoolListComponent,
        CoolCardComponent,
        CdkVirtualScrollViewport,
    ],
    providers: [],
    bootstrap: [AppComponent],
})
export class AppModule {
}
