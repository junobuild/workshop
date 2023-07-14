import {
  computed,
  Injectable,
  Signal,
  signal,
  WritableSignal,
} from '@angular/core';
import { authSubscribe, User, type Unsubscribe } from '@junobuild/core';

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  user: WritableSignal<User | undefined | null> = signal(undefined);
  readonly signedIn: Signal<boolean> = computed(
    () => this.user() !== null && this.user() !== undefined
  );

  constructor() {
    // TODO: STEP_4_AUTH_SUBSCRIBE
    // authSubscribe((user) => this.user.set(user));
  }
}
