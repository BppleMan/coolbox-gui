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
import {BehaviorSubject, map, Observable} from "rxjs"
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
    cools$: BehaviorSubject<Cool[]> = new BehaviorSubject<Cool[]>([])
    // map是为了在全局事件监听中调度使用，因为后端返回的event只会带上cool_name
    cool_map$: Observable<Map<string, Cool>> = this.cools$.pipe(
        map(cools => {
            let cool_map: Map<string, Cool> = new Map<string, Cool>()
            cools.forEach((cool) => {
                cool_map.set(cool.name, cool)
            })
            return cool_map
        }),
    )
    selected_count: number = 0

    constructor(private cool_service: CoolService) {
    }

    ngOnInit() {
        this.cool_service.cool_list().then((cools: Cool[]) => {
            this.cools$.next(cools.map((cool) => new Cool(cool)))
        })
        .catch((err) => {
            // TODO show toast
        })
        this.cool_map$.subscribe((cool_map: Map<string, Cool>) => {
            cool_map.forEach((cool) => {
                cool.selected.subscribe((selected) => {
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
        return this.selected_count > 0 && this.selected_count === this.cools$.value.length
    }

    get some_selected(): boolean {
        return this.selected_count > 0 && !this.all_selected
    }

    toggleSelectAll(all_selected: boolean) {
        if (all_selected) {
            this.cools$.value.forEach((cool) => cool.selected.next(true))
        } else {
            this.cools$.value.forEach((cool) => cool.selected.next(false))
        }
    }
}
