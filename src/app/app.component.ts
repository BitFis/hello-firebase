import { Component, inject, OnInit } from '@angular/core';
import { NgbAlertModule } from '@ng-bootstrap/ng-bootstrap';
import { NgIf } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { Alert, AlertService } from '@lib/services';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, NgIf, NgbAlertModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
})
export class AppComponent implements OnInit {
  alerts: Alert[] = [];
  title = 'welcome firebase';

  _alert = inject(AlertService);

  ngOnInit() {
    this._alert.alerts$.subscribe((value) => {
      if (value) {
        this.alerts.push(value);
      }
    });
  }

  close(alert: Alert) {
    this.alerts.splice(this.alerts.indexOf(alert), 1);
  }
}
