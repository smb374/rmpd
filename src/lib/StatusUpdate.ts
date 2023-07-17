import { writable, type Writable } from "svelte/store";
import type { Status, SingleMode, PlayState } from "./bindings";
import { tvalToMs, updateState } from "./Utils";

const currentTime = writable(0);
const totalItems = writable(0);
const totalTime = writable(0);
const isPlaying = writable(false);
const isRepeat = writable(false);
const singleMode: Writable<SingleMode> = writable("Disabled");
const playState: Writable<PlayState> = writable("Paused");
let intervalID: NodeJS.Timer = undefined;

function statusUpdate(status: Status) {
  if (status === undefined) {
    return;
  }
  updateState(totalItems, status?.playlist_length);
  updateState(isRepeat, status?.repeat);
  updateState(singleMode, status?.single);
  updateState(totalTime, tvalToMs(status?.duration));
  updateState(currentTime, tvalToMs(status?.elapsed));
  updateState(playState, status?.state);
  if (status?.state === "Playing") {
    if (intervalID === undefined) {
      intervalID = setInterval(() => currentTime.update((n) => n + 1000), 1000);
    }
  } else {
    if (intervalID !== undefined) {
      clearInterval(intervalID);
      intervalID = undefined;
    }
  }
}

export {
  currentTime,
  totalItems,
  totalTime,
  isPlaying,
  isRepeat,
  singleMode,
  playState,
  statusUpdate,
};
