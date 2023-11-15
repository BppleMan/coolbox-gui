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
import {BehaviorSubject, map} from "rxjs"
import {CoolCardComponent} from "../cool-card/cool-card.component"
import {Cool, CoolState} from "../model/models"
import {CoolService} from "../service/cool.service"
import {TranslateModule} from "@ngx-translate/core"
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
        TranslateModule
    ],
    templateUrl: "./cool-list.component.html",
    styleUrls: ["./cool-list.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CoolListComponent implements OnInit {
    cools$: BehaviorSubject<Cool[]> = new BehaviorSubject<Cool[]>([])
    // map是为了在全局事件监听中调度使用，因为后端返回的event只会带上cool_name
    cool_map$: BehaviorSubject<Map<string, Cool>> = new BehaviorSubject<Map<string, Cool>>(new Map<string, Cool>())
    selected_count: number = 0

    constructor(private cool_service: CoolService) {
    }

    ngOnInit() {
        // create a middle observable to transform cools to cool_map
        const middleObservable = this.cools$.pipe(
            map(cools => {
                let cool_map: Map<string, Cool> = new Map<string, Cool>()
                cools.forEach((cool) => {
                    cool_map.set(cool.name, cool)
                })
                return cool_map
            }),
        )

        // get cools from backend
        this.cool_service.cool_list().then((cools: Cool[]) => {
            this.cools$.next(cools.map((cool) => new Cool(cool)))
        })
        .catch((err) => {
            // TODO show toast
        })

        // if middleObservable changed, then update cool_map$
        middleObservable.subscribe((cool_map: Map<string, Cool>) => {
            this.cool_map$.next(cool_map)
        })

        // if cool_map$ changed, then update selected_count
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

        // listen task event, empty then for non-block
        this.cool_service.listen_task_event(this.cool_map$).then()
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
            // test for cool state change
            this.cools$.value[0].state = CoolState.Installing
        } else {
            this.cools$.value.forEach((cool) => cool.selected.next(false))
        }
    }
}
