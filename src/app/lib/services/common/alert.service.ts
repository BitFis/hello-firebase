import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';

export interface Alert {
  type: string;
  message: string;
  details?: string;
}

@Injectable({
  providedIn: 'root',
})
export class AlertService {
  alerts$ = new BehaviorSubject<Alert | null>(null);

  error(message: string, details?: string) {
    this.alerts$.next({
      type: 'danger',
      message: message,
      details: details,
    });
  }
}
