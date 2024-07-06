import { Component, inject } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from '@lib/services';

@Component({
  standalone: true,
  templateUrl: './home.component.html',
})
export class HomeComponent {
  private readonly _router = inject(Router);
  private readonly _authService = inject(AuthService);

  logout() {
    this._authService.logout();

    this._router.navigate([`auth/login`]);
  }
}
