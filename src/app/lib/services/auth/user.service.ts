import { Injectable } from '@angular/core';
import { User } from '@angular/fire/auth';
import { BehaviorSubject } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class UserService {
  $user = new BehaviorSubject<User | null>(null);

  get(): User | null {
    return this.$user.getValue();
  }

  set(user: User | null): void {
    this.$user.next(user);
  }

  clear(): void {
    this.$user.next(null);
  }
}
