import { HttpClient, provideHttpClient } from "@angular/common/http"
import { ApplicationConfig, importProvidersFrom, NgZone } from "@angular/core"
import {
    MatDialogModule
} from "@angular/material/dialog"
import { provideRouter } from "@angular/router"
import { TranslateLoader, TranslateModule } from "@ngx-translate/core"
import { TranslateHttpLoader } from "@ngx-translate/http-loader"
import { HIGHLIGHT_OPTIONS, HighlightModule } from "ngx-highlightjs"
import { provideAnimations } from "@angular/platform-browser/animations"

import { routes } from "./app.routes"

export const app_config: ApplicationConfig = {
    providers: [
        provideRouter(routes),
        provideHttpClient(),
        provideAnimations(),
        importProvidersFrom(
            TranslateModule.forRoot({
                loader: {
                    provide: TranslateLoader,
                    useFactory: HttpLoaderFactory,
                    deps: [HttpClient],
                },
            }),
        ),
        importProvidersFrom(MatDialogModule),
        importProvidersFrom(NgZone),
        importProvidersFrom(HighlightModule),
        {
            provide: HIGHLIGHT_OPTIONS,
            useValue: {
                coreLibraryLoader: () => import("highlight.js/lib/core"),
                languages: {
                    typescript: () =>
                        import("highlight.js/lib/languages/typescript"),
                    bash: () => import("highlight.js/lib/languages/bash"),
                    // css: () => import("highlight.js/lib/languages/css"),
                    // xml: () => import("highlight.js/lib/languages/xml"),
                    // bash: () => import("highlight.js/lib/languages/bash"),
                },
            },
        },
    ],
}

export function HttpLoaderFactory(http: HttpClient) {
    return new TranslateHttpLoader(http)
}
