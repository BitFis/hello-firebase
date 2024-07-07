import { Component, inject } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService, UserService } from '@lib/services';

@Component({
  standalone: true,
  templateUrl: './home.component.html',
})
export class HomeComponent {
  $user = inject(UserService).$user;

  private readonly _router = inject(Router);
  private readonly _authService = inject(AuthService);

  logout() {
    this._authService.logout();

    this._router.navigate([`auth/login`]);
  }
}
