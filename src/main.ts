import { bootstrapApplication } from "@angular/platform-browser"
import { app_config } from "./app/app.config"
import { AppComponent } from "./app/app.component"

bootstrapApplication(AppComponent, app_config).catch((err) =>
    console.error(err),
)
