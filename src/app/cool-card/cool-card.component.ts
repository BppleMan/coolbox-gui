import {CommonModule} from "@angular/common"
import {Component, EventEmitter, Input, OnChanges, Output, ViewChild, Inject} from "@angular/core"
import {MatButtonModule} from "@angular/material/button"
import {MatCheckboxModule} from "@angular/material/checkbox"
import {MatRippleModule} from "@angular/material/core"
import {MatExpansionModule, MatExpansionPanel} from "@angular/material/expansion"
import {MatIconModule} from "@angular/material/icon"
import {MatStepperModule} from "@angular/material/stepper"
import {MatDividerModule} from "@angular/material/divider"
import {MatProgressBarModule} from "@angular/material/progress-bar"
import {BehaviorSubject} from "rxjs"
import {Cool, CoolListItem} from "../model/models"
import {HIGHLIGHT_OPTIONS, HighlightModule} from "ngx-highlightjs"
import {CoolService} from "../service/cool.service"
import {MatDialog, MAT_DIALOG_DATA, MatDialogRef, MatDialogModule} from '@angular/material/dialog';
import {FormsModule} from '@angular/forms';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';

export interface DialogData {
    password: string;
    name: string;
  }
@Component({
    selector: "app-cool-card",
    standalone: true,
    imports: [CommonModule, MatButtonModule, MatCheckboxModule, MatExpansionModule, MatIconModule, MatStepperModule, MatRippleModule, HighlightModule, MatDividerModule, MatProgressBarModule, MatDialogModule],
    templateUrl: "./cool-card.component.html",
    styleUrls: ["./cool-card.component.scss"],
    providers: [
        {
            provide: HIGHLIGHT_OPTIONS,
            useValue: {
                coreLibraryLoader: () => import("highlight.js/lib/core"),
                languages: {
                    typescript: () => import("highlight.js/lib/languages/typescript"),
                    bash: () => import("highlight.js/lib/languages/bash"),
                },
            },
        },
    ]
})
export class CoolCardComponent {
    @ViewChild("expansionPanel") panel!: MatExpansionPanel
    @Input() cool!: Cool
    @Input() selected!: BehaviorSubject<boolean>
    expanded = false
    consoleCode = 'mkdir xxx-project && cp a b'
    password = ''
    constructor(private cool_service: CoolService, public dialog: MatDialog) {
    }
    openDialog(): void {
        const dialogRef = this.dialog.open(PromptDialog, {
          data: {password: this.password},
        });
    
        dialogRef.afterClosed().subscribe(result => {
          console.log('The dialog was closed, password is', result);

        });
      }
    toggle_select(event: MouseEvent) {
        event.preventDefault()
        event.stopPropagation()
        this.selected.next(!this.selected.value)
    }

    toggle_panel(event: MouseEvent) {
        console.log('toggle_selected')
        event.preventDefault()
        event.stopPropagation()
        this.panel.toggle()
    }

    setOpened() {
        this.expanded = true
    }

    setClosed() {
        this.expanded = false
    }

    copy(content: string) {
        console.log('copy', content)
        navigator.clipboard.writeText(content)
        // TODO we should show toast here to indicate the copy action
    }

    install(event: MouseEvent) {
        event.stopPropagation()
        // this.cool_service.install_cool([this.cool]).then()
        // TODO only for testing dialog
        this.openDialog()
    }
    
}
@Component({
    selector: 'prompt-dialog',
    templateUrl: 'prompt-dialog.html',
    standalone: true,
    imports: [MatDialogModule, MatFormFieldModule, MatInputModule, FormsModule, MatButtonModule],
  })
  export class PromptDialog {
    constructor(
      public dialogRef: MatDialogRef<PromptDialog>,
      @Inject(MAT_DIALOG_DATA) public data: DialogData,
    ) {}
  
    onNoClick(): void {
      this.dialogRef.close();
    }
  }
