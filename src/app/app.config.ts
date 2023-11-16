import { HttpClient } from "@angular/common/http";
import { ApplicationConfig, importProvidersFrom } from "@angular/core";
import { provideRouter } from "@angular/router";
import { TranslateLoader, TranslateModule } from "@ngx-translate/core";
import { TranslateHttpLoader } from "@ngx-translate/http-loader";
import { HIGHLIGHT_OPTIONS } from "ngx-highlightjs";

import { routes } from "./app.routes";

export const appConfig: ApplicationConfig = {
    providers: [
        provideRouter(routes),
        importProvidersFrom(
            TranslateModule.forRoot({
                loader: {
                    provide: TranslateLoader,
                    useFactory: HttpLoaderFactory,
                    deps: [HttpClient],
                },
            }),
        ),
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
};

function HttpLoaderFactory(http: HttpClient) {
    return new TranslateHttpLoader(http);
}
