import {CdkFixedSizeVirtualScroll, CdkVirtualForOf, CdkVirtualScrollViewport} from "@angular/cdk/scrolling"
import {CommonModule} from "@angular/common"
import {ChangeDetectionStrategy, Component, OnInit} from "@angular/core"
import {MatButtonModule} from "@angular/material/button"
import {MatButtonToggleModule} from "@angular/material/button-toggle"
import {MatCardModule} from "@angular/material/card"
import {MatCheckboxModule} from "@angular/material/checkbox"
import {MatDividerModule} from "@angular/material/divider"
import {MatExpansionModule} from "@angular/material/expansion"
import {MatListModule} from "@angular/material/list"
import {BehaviorSubject} from "rxjs"
import {CoolCardComponent} from "../cool-card/cool-card.component"
import {Cool} from "../model/models"
import {CoolService} from "../service/cool.service"

@Component({
    selector: "app-cool-list",
    standalone: true,
    imports: [
        CommonModule,
        MatExpansionModule,
        MatCardModule,
        MatButtonToggleModule,
        MatButtonModule,
        MatDividerModule,
        MatCheckboxModule,
        CoolCardComponent,
        MatListModule,
        CdkVirtualScrollViewport,
        CdkFixedSizeVirtualScroll,
        CdkVirtualForOf,
    ],
    templateUrl: "./cool-list.component.html",
    styleUrls: ["./cool-list.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CoolListComponent implements OnInit {
    cool_list$: BehaviorSubject<Cool[]> = new BehaviorSubject<Cool[]>([])
    selected_cools: BehaviorSubject<boolean>[] = []
    selected_count: number = 0

    constructor(private cool_service: CoolService) {
    }

    ngOnInit() {
        this.cool_service.cool_list().then((cool_list: Cool[]) => {
            this.cool_list$.next(cool_list)
        })
        .catch((err) => {
            // TODO show toast
        })
        this.cool_list$.subscribe((cool_list: Cool[]) => {
            this.selected_cools.forEach((selected_cools) => {
                selected_cools.unsubscribe()
            })
            this.selected_cools = cool_list.map((cool) => new BehaviorSubject(false))
            this.selected_cools.forEach((selected_cools) => {
                selected_cools.subscribe((selected) => {
                    if (selected) {
                        this.selected_count += 1
                    } else if (this.selected_count > 0) {
                        this.selected_count -= 1
                    }
                })
            })
        })
    }

    get all_selected(): boolean {
        return this.selected_count > 0 && this.selected_count === this.selected_cools.length
    }

    get some_selected(): boolean {
        return this.selected_count > 0 && !this.all_selected
    }

    toggleSelectAll(all_selected: boolean) {
        if (all_selected) {
            this.selected_cools.forEach((selected) => selected.next(true))
        } else {
            this.selected_cools.forEach((selected) => selected.next(false))
        }
    }
}
