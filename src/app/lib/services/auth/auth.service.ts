import { Injectable, inject } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { AuthProviders, FirebaseAuth } from '@lib/backend/firebase/auth';

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  _auth = inject(FirebaseAuth);

  isAuthenticated$ = new BehaviorSubject<boolean>(false);

  constructor() {
    this._auth.updateAuthentication().then((authenticated) => {
      this.isAuthenticated$.next(authenticated);
    });
  }

  async check(): Promise<boolean> {
    const authenticated = await this._auth.updateAuthentication();
    this.isAuthenticated$.next(authenticated);
    return authenticated;
  }

  get isAuthenticated(): boolean {
    return this.isAuthenticated$.getValue();
  }

  async loginWithProvider(provider: AuthProviders): Promise<void> {
    await this._auth.loginWithProvider(provider);
    this.isAuthenticated$.next(true);
  }

  login(): void {
    this.isAuthenticated$.next(true);
  }

  logout(): void {
    this._auth.logout();
    this.isAuthenticated$.next(false);
  }
}
