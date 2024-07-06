import { Routes } from '@angular/router';

export const routes: Routes = [
  {
    path: 'auth',
    loadChildren: async () => (await import('./pages/auth')).routes,
  },
  {
    path: '',
    loadComponent: async () =>
      (await import('./pages/home/home.component')).HomeComponent,
  },
  {
    path: '**',
    loadComponent: async () =>
      (await import('./pages/screens/notfound/not-found.component'))
        .NotFoundComponent,
  },
];
