import { Component, inject, Input, ViewEncapsulation } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from '@lib/services';

@Component({
  standalone: true,
  templateUrl: './login.component.html',
  styleUrl: './login.component.css',
  encapsulation: ViewEncapsulation.None,
})
export class LoginComponent {
  @Input() returnUrl!: string;

  private readonly _authService = inject(AuthService);
  private readonly _router = inject(Router);

  login() {
    this._authService.login();

    this._router.navigate(['/']);
  }
}
