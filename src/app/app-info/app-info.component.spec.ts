import {ComponentFixture, TestBed} from "@angular/core/testing"

import {AppInfoComponent} from "./app-info.component"

describe("AppInfoComponent", () => {
    let component: AppInfoComponent
    let fixture: ComponentFixture<AppInfoComponent>

    beforeEach(() => {
        TestBed.configureTestingModule({
            declarations: [AppInfoComponent],
        })
        fixture = TestBed.createComponent(AppInfoComponent)
        component = fixture.componentInstance
        fixture.detectChanges()
    })

    it("should create", () => {
        expect(component).toBeTruthy()
    })
})
