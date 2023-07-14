import { NgIf } from '@angular/common';
import { Component, inject } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatDialog, MatDialogModule } from '@angular/material/dialog';
import { AuthService } from '../../services/auth.service';
import { DocsService } from '../../services/docs.service';
import { ModalComponent } from '../modal/modal.component';
import { TableComponent } from '../table/table.component';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss'],
  standalone: true,
  imports: [
    NgIf,
    MatButtonModule,
    ModalComponent,
    MatDialogModule,
    TableComponent,
  ],
})
export class HomeComponent {
  private readonly authService = inject(AuthService);
  private dialog = inject(MatDialog);
  private docsService = inject(DocsService);
  readonly signedIn = this.authService.signedIn;

  // TODO: STEP_2_AUTH_SIGN_IN
  readonly signIn = () => undefined;
  // readonly signIn = signIn;
  // TODO: STEP_3_AUTH_SIGN_OUT
  readonly signOut = () => undefined;
  // readonly signOut = signOut;

  openModal() {
    let dialogRef = this.dialog.open(ModalComponent, {
      height: '400px',
      width: '600px',
    });

    dialogRef.afterClosed().subscribe(() => this.docsService.reload());
  }
}
