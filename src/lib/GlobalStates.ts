import { writable, type Writable } from "svelte/store";
import type { SongInQueue, Status } from "./bindings";

const currentQueue: Writable<SongInQueue[]> = writable([]);
const currentSong: Writable<SongInQueue> = writable(undefined);
const currentStatus: Writable<Status> = writable(undefined);

export { currentQueue, currentSong, currentStatus };
