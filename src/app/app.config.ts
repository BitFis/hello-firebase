import { ApplicationConfig, provideZoneChangeDetection } from '@angular/core';
import { provideAuth, getAuth } from '@angular/fire/auth';
import { provideFirebaseApp, initializeApp } from '@angular/fire/app';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';

export const appConfig: ApplicationConfig = {
  providers: [
    provideFirebaseApp(() =>
      initializeApp({
        apiKey: 'AIzaSyA3ApoNvJNLB1p1eJA6wVK2P7UU_r2kPH8',
        authDomain: 'welcome-firebase-62ffd.firebaseapp.com',
        projectId: 'welcome-firebase-62ffd',
        storageBucket: 'welcome-firebase-62ffd.appspot.com',
        messagingSenderId: '979724850902',
        appId: '1:979724850902:web:e3dc13045d97cea8c99b77',
      }),
    ),
    provideAuth(() => getAuth()),
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(routes),
  ],
};
