<script lang="ts">
    import MainTab from "./lib/MainTab.svelte";
    import Navigation from "./lib/Navigation.svelte";
    import Queue from "./lib/Queue.svelte";
    import {
        currentQueue,
        currentSong,
        currentStatus,
    } from "./lib/GlobalStates";
    import musicNote from "@iconify/icons-mdi/music-note";
    import playlistMusic from "@iconify/icons-mdi/playlist-music";
    import type { IconifyIcon } from "@iconify/svelte";
    import { currentSongUpdate } from "./lib/CurrentSongUpdate";
    import { statusUpdate } from "./lib/StatusUpdate";
    import { updateStatePromise } from "./lib/Utils";
    import { currentsong, playlistinfo, status } from "./lib/bindings";
    import { listen } from "@tauri-apps/api/event";

    // initial gather.
    updateStatePromise(currentQueue, playlistinfo());
    updateStatePromise(currentSong, currentsong());
    updateStatePromise(currentStatus, status());
    // update state by event.
    listen("queue", () => {
        updateStatePromise(currentQueue, playlistinfo());
    });
    listen("currentsong", () => {
        updateStatePromise(currentSong, currentsong());
    });
    listen("status", () => {
        updateStatePromise(currentStatus, status());
    });

    $: currentSongUpdate($currentSong);
    $: statusUpdate($currentStatus);
    const triggers: IconifyIcon[] = [musicNote, playlistMusic];
    const contents = [
        { component: MainTab, props: {} },
        { component: Queue, props: {} },
    ];
</script>

<main class="mocha flex flex-col w-screen h-screen relative">
    <Navigation {triggers} {contents} />
</main>
