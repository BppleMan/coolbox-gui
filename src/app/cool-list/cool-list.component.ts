import {CdkFixedSizeVirtualScroll, CdkVirtualForOf, CdkVirtualScrollViewport} from "@angular/cdk/scrolling"
import {CommonModule} from "@angular/common"
import {ChangeDetectionStrategy, ChangeDetectorRef, Component, OnInit} from "@angular/core"
import {MatButtonModule} from "@angular/material/button"
import {MatButtonToggleModule} from "@angular/material/button-toggle"
import {MatCardModule} from "@angular/material/card"
import {MatCheckboxModule} from "@angular/material/checkbox"
import {MatDividerModule} from "@angular/material/divider"
import {MatExpansionModule} from "@angular/material/expansion"
import {MatListModule} from "@angular/material/list"
import {BehaviorSubject, Observable} from "rxjs"
import {CoolCardComponent} from "../cool-card/cool-card.component"
import {Cool, CoolListItem} from "../model/models"
import {CoolService} from "../service/cool.service"

@Component({
    selector: "app-cool-list",
    standalone: true,
    imports: [CommonModule, MatExpansionModule, MatCardModule, MatButtonToggleModule, MatButtonModule, MatDividerModule, MatCheckboxModule, CoolCardComponent, MatListModule, CdkVirtualScrollViewport, CdkFixedSizeVirtualScroll, CdkVirtualForOf],
    templateUrl: "./cool-list.component.html",
    styleUrls: ["./cool-list.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CoolListComponent implements OnInit {
    cool_list$!: Observable<Cool[]>;
    all_cool_list$ = new BehaviorSubject<CoolListItem[]>([]);
    allSelected = false;
    constructor(private cool_service: CoolService) {
    }

    async ngOnInit() {
        this.cool_list$ = this.cool_service.cool_list();
        this.cool_list$.subscribe((cool_list: Cool[]) => {
            console.log("cool_list", cool_list);
            const newAllCoolList = cool_list.map((cool: Cool) => ({item: cool, selected: false}));
            this.all_cool_list$.next(newAllCoolList);
        });
    }
    get selectedCount(): number {
        return this.all_cool_list$.value.filter(item => item.selected).length;
    }
    get someSelected() {
        return this.selectedCount > 0 && !this.allSelected;
    }
    updateSelectedCount(selectedItem: CoolListItem) {
        // update the all_cool_list array
        const newAllCoolList = this.all_cool_list$.value.map(item => {
            if (item.item.name === selectedItem.item.name) {
                return { ...item, selected: selectedItem.selected };
            }
            return item;
        });
        this.all_cool_list$.next(newAllCoolList);
    }
    selectAll(isSelected: boolean) {
        this.allSelected = isSelected;
        const newAllCoolList = this.all_cool_list$.value.map(item => ({ ...item, selected: isSelected }));
        this.all_cool_list$.next(newAllCoolList);
    }
    
}
