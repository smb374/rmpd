import { writable } from "svelte/store";
import type { Status } from "./bindings";
import { tvalToMs, updateState } from "./Utils";

const currentTime = writable(0);
const totalItems = writable(0);
const totalTime = writable(0);
const isPlaying = writable(false);
const isRepeat = writable(false);
const isSingle = writable(false);
let intervalID: NodeJS.Timer = undefined;

function statusUpdate(status: Status) {
    updateState(totalItems, status?.playlist_length);
    updateState(isRepeat, status?.repeat);
    updateState(
        isSingle,
        status?.single !== undefined && status?.single !== "Disabled",
    );
    updateState(totalTime, tvalToMs(status?.duration));
    updateState(currentTime, tvalToMs(status?.elapsed));
    if (status?.state === "Playing") {
        if (intervalID === undefined) {
            intervalID = setInterval(
                () => currentTime.update((n) => n + 1000),
                1000,
            );
        }
        isPlaying.set(true);
    } else {
        if (intervalID !== undefined) {
            clearInterval(intervalID);
            intervalID = undefined;
        }
        isPlaying.set(false);
    }
}

export {
    currentTime,
    totalItems,
    totalTime,
    isPlaying,
    isRepeat,
    isSingle,
    statusUpdate,
};
