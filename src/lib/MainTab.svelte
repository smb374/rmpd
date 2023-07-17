<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import {
        coverPath,
        currentTitle,
        currentArtist,
        currentAlbum,
    } from "./CurrentSongUpdate";
    import Cover from "./Cover.svelte";

    $: src = convertFileSrc($coverPath);
</script>

<div class="flex-1 flex flex-row space-x-8 items-center">
    <Cover {src} />
    <div class="prose text-start">
        <h2 class="font-bold">{$currentTitle}</h2>
        <p class="font-normal">{$currentArtist}</p>
        <p class="font-normal text-sm">{$currentAlbum}</p>
    </div>
</div>
