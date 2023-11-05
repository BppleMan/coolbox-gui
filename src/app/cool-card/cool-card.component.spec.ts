import {ComponentFixture, TestBed} from "@angular/core/testing"

import {CoolCardComponent} from "./cool-card.component"

describe("CoolCardComponent", () => {
    let component: CoolCardComponent
    let fixture: ComponentFixture<CoolCardComponent>

    beforeEach(() => {
        TestBed.configureTestingModule({
            imports: [CoolCardComponent],
        })
        fixture = TestBed.createComponent(CoolCardComponent)
        component = fixture.componentInstance
        fixture.detectChanges()
    })

    it("should create", () => {
        expect(component).toBeTruthy()
    })
})
