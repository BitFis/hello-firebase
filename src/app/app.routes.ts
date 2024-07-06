import { Routes } from '@angular/router';
import { authGuard } from '@lib/guards';

export const routes: Routes = [
  {
    path: 'auth',
    loadChildren: async () => (await import('@pages/auth')).routes,
    canMatch: [authGuard({ requiresAuthentication: false })],
  },
  {
    path: '',
    loadComponent: async () =>
      (await import('@pages/home/home.component')).HomeComponent,
    canMatch: [authGuard({ requiresAuthentication: true })],
  },
  {
    path: '**',
    loadComponent: async () =>
      (await import('@pages/screens/notfound/not-found.component'))
        .NotFoundComponent,
    canMatch: [authGuard({ requiresAuthentication: true })],
  },
];
