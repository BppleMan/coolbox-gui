import {registerLocaleData} from "@angular/common"
import {HttpClientModule} from "@angular/common/http"
import zh from "@angular/common/locales/zh"
import {NgModule} from "@angular/core"
import {FormsModule} from "@angular/forms"
import {BrowserModule} from "@angular/platform-browser"
import {BrowserAnimationsModule} from "@angular/platform-browser/animations"
import {NzButtonModule} from "ng-zorro-antd/button"
import {NzCollapseModule} from "ng-zorro-antd/collapse"
import {NzDividerModule} from "ng-zorro-antd/divider"
import {NZ_I18N, zh_CN} from "ng-zorro-antd/i18n"
import {NzLayoutModule} from "ng-zorro-antd/layout"
import {NzPageHeaderModule} from "ng-zorro-antd/page-header"
import {NzPopoverModule} from "ng-zorro-antd/popover"
import {NzStepsModule} from "ng-zorro-antd/steps"
import {AppInfoComponent} from "./app-info/app-info.component"

import {AppComponent} from "./app.component"
import {CoolListComponent} from "./cool-list/cool-list.component"

registerLocaleData(zh)

@NgModule({
    declarations: [
        AppComponent,
        AppInfoComponent,
        CoolListComponent,
    ],
    imports: [
        BrowserModule,
        FormsModule,
        HttpClientModule,
        BrowserAnimationsModule,
        NzButtonModule,
        NzLayoutModule,
        NzPageHeaderModule,
        NzCollapseModule,
        NzDividerModule,
        NzStepsModule,
        NzPopoverModule,
    ],
    providers: [
        {provide: NZ_I18N, useValue: zh_CN},
    ],
    bootstrap: [AppComponent],
})
export class AppModule {
}
