<script lang="ts">
    import MainTab from "./lib/MainTab.svelte";
    import Navigation from "./lib/Navigation.svelte";
    import Queue from "./lib/Queue.svelte";
    import { setup, unlistenAll } from "./lib/GlobalStates";
    import musicNote from "@iconify/icons-mdi/music-note";
    import playlistMusic from "@iconify/icons-mdi/playlist-music";
    import type { IconifyIcon } from "@iconify/svelte";
    import { onDestroy } from "svelte";

    const unlistens = setup();

    const triggers: IconifyIcon[] = [musicNote, playlistMusic];
    const contents = [
        { component: MainTab, props: {} },
        { component: Queue, props: {} },
    ];

    onDestroy(() => {
        unlistenAll(unlistens);
    });
</script>

<main class="mocha flex flex-col w-screen h-screen relative">
    <Navigation {triggers} {contents} />
</main>
