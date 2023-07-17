import { writable, type Writable } from "svelte/store";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import {
    currentsong,
    status,
    playlistinfo,
    type SongInQueue,
    type Status,
} from "./bindings";
import { currentSongUpdate } from "./CurrentSongUpdate";
import { statusUpdate } from "./StatusUpdate";
import { stateSubscription, updateStatePromise } from "./Utils";

const currentQueue: Writable<SongInQueue[]> = writable([]);
const currentSong: Writable<SongInQueue> = writable(undefined);
const currentStatus: Writable<Status> = writable(undefined);
const listenEvents: UnlistenFn[] = [];

function setup() {
    // set subscriptions.
    // stateSubscription(currentQueue, (queue: SongInQueue[]) =>
    //     console.log(queue),
    // );
    stateSubscription(currentSong, currentSongUpdate);
    stateSubscription(currentStatus, statusUpdate);
    // initial gather.
    updateStatePromise(currentQueue, playlistinfo());
    updateStatePromise(currentSong, currentsong());
    updateStatePromise(currentStatus, status());
    // update state by event.
    listen("queue", () => {
        updateStatePromise(currentQueue, playlistinfo());
    }).then((x) => listenEvents.push(x));
    listen("currentsong", () => {
        updateStatePromise(currentSong, currentsong());
    }).then((x) => listenEvents.push(x));
    listen("status", () => {
        updateStatePromise(currentStatus, status());
    }).then((x) => listenEvents.push(x));
}

function unlistenAll() {
    console.log(listenEvents.length, " event to unlisten...");
    listenEvents.forEach((x) => x());
    console.log("Done.");
}

export { setup, unlistenAll, currentQueue };
