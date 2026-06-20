export class PersistedState<T> {
  #key: string;
  #value = $state<T>() as T;

  constructor(key: string, initial: T) {
    this.#key = key;
    const stored = localStorage.getItem(key);
    this.#value = stored !== null ? (JSON.parse(stored) as T) : initial;
  }

  get current(): T {
    return this.#value;
  }

  set current(value: T) {
    this.#value = value;
    localStorage.setItem(this.#key, JSON.stringify(value));
  }
}
