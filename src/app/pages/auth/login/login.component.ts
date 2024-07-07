import {
  Component,
  inject,
  Input,
  OnInit,
  ViewEncapsulation,
} from '@angular/core';
import { Router } from '@angular/router';
import { AlertService, AuthService, UserService } from '@lib/services';
import { Auth } from '@angular/fire/auth';
import { AuthProviders } from '@lib/backend/firebase/auth';

@Component({
  standalone: true,
  templateUrl: './login.component.html',
  styleUrl: './login.component.css',
  encapsulation: ViewEncapsulation.None,
})
export class LoginComponent implements OnInit {
  @Input() returnUrl!: string;

  private readonly _authService = inject(AuthService);
  private readonly _router = inject(Router);
  private readonly _auth = inject(Auth);
  private readonly _userService = inject(UserService);
  private readonly _alert = inject(AlertService);

  errorHandler(err: Error): void {
    console.error('auth failed: ', err);
    this._alert.error('authentication failed ...', String(err));
  }

  ngOnInit() {
    this._authService.check().then((authenticated) => {
      this._userService.set(this._auth.currentUser);
      if (authenticated) {
        this._router.navigate(['/']);
      }
    });
  }

  loginProvider(provider: AuthProviders): void {
    this._authService
      .loginWithProvider(provider)
      .then(() => {
        this._router.navigate(['/']);
      })
      .catch(this.errorHandler.bind(this));
  }

  login() {
    this._authService
      .login('', '')
      .then(() => {
        this._router.navigate(['/']);
      })
      .catch(this.errorHandler.bind(this));
  }
}
