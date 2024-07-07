import { inject, Injectable } from '@angular/core';
import { Auth, browserSessionPersistence } from '@angular/fire/auth';
import { OAuthProvider, signInWithPopup } from '@angular/fire/auth';
import { UserService } from '@lib/services';

export type AuthProviders = 'microsoft';

@Injectable({
  providedIn: 'root',
})
export class FirebaseAuth {
  private readonly _auth = inject(Auth);
  private readonly _userService = inject(UserService);

  private readonly msProvider = new OAuthProvider('microsoft.com');

  /**
   * Update if a user is authenticated
   * @returns true if user was authenticated
   *          false currently no user is authenticated
   */
  async updateAuthentication(): Promise<boolean> {
    await this._auth.setPersistence(browserSessionPersistence);
    return this._auth.currentUser != null;
  }

  private async _loginWithProvider(provider: OAuthProvider): Promise<void> {
    const result = await signInWithPopup(this._auth, provider);
    const credential = OAuthProvider.credentialFromResult(result);
    if (!credential) {
      throw new Error('Failed to retrieve the credentials');
    }

    console.info('Log in successful');
    this._userService.set(this._auth.currentUser);
  }

  /**
   * Initiate login
   * @param provider the provider
   */
  async loginWithProvider(provider: AuthProviders): Promise<void> {
    switch (provider) {
      case 'microsoft': {
        return this._loginWithProvider(this.msProvider);
      }
      default:
        throw new Error(`Provider '${provider}' not supported`);
    }
  }

  login(_user: string, _password: string): Promise<void> {
    throw Error('Login not supported');
  }

  async logout(): Promise<void> {
    await this._auth.signOut();
  }
}
