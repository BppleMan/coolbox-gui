import { Component, Inject} from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { MatDialogRef, MatDialogModule, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatButtonModule } from '@angular/material/button';
import { ReactiveFormsModule } from '@angular/forms';
import {FormsModule} from '@angular/forms';
import { TranslateModule } from '@ngx-translate/core';

@Component({
    selector: 'app-password-dialog',
    standalone: true,
    template: `
    <h1 mat-dialog-title >{{'DIALOG.PASSWORD_TITLE' | translate}}</h1>
    <!-- <h1 mat-dialog-title>{{data.title}}</h1> -->
    <div mat-dialog-content>
      hello
      <!-- <mat-form-field>
        <input matInput [formControl]="passwordControl" type="password">
      </mat-form-field> -->
    </div>
    <div mat-dialog-actions align="end">
      <button mat-button (click)="onNoClick()">Cancel</button>
      <button mat-button [mat-dialog-close]="passwordControl.value" cdkFocusInitial>Ok</button>
    </div>
  `,
    imports: [MatFormFieldModule, MatInputModule, MatButtonModule, ReactiveFormsModule, FormsModule, MatDialogModule, TranslateModule],
  
})
export class PasswordDialogComponent {
    passwordControl = new FormControl('', Validators.required);
  @Inject(MAT_DIALOG_DATA)
    public data: {title: string} = {title: "title"};

  constructor(public dialogRef: MatDialogRef<PasswordDialogComponent>) {}

  onNoClick(): void {
      this.dialogRef.close();
  }
}