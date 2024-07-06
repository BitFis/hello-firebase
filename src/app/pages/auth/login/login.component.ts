import { Component, Input, ViewEncapsulation } from '@angular/core';

@Component({
  standalone: true,
  templateUrl: './login.component.html',
  styleUrl: './login.component.css',
  encapsulation: ViewEncapsulation.None,
})
export class LoginComponent {
  @Input() returnUrl!: string;
}
