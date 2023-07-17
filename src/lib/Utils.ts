import type { Writable } from "svelte/store";
import type { TVal } from "./bindings";

function updateStatePromise<T>(state: Writable<T>, data: Promise<T>) {
  data
    .then((value: T) => updateState(state, value))
    .catch((err) => console.error(err));
}

function updateState<T>(state: Writable<T>, data: T) {
  state.update((x) => {
    if (data === undefined) {
      return x;
    } else {
      return data;
    }
  });
}

function stateSubscription<T>(state: Writable<T>, callback: (arg: T) => void) {
  state.subscribe((x: T) => {
    if (x !== undefined) {
      callback(x);
    }
  });
}

function tvalToMs(value: TVal): number {
  if (value !== undefined) {
    return value.secs * 1000 + value.nanos / 1000000;
  } else {
    return 0;
  }
}

export { stateSubscription, tvalToMs, updateState, updateStatePromise };
