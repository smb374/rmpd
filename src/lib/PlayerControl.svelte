<script lang="ts">
    import {
        currentItem,
        currentTitle,
        currentArtist,
        currentAlbum,
    } from "./CurrentSongUpdate";
    import {
        currentTime,
        totalItems,
        totalTime,
        isPlaying,
        isRepeat,
        singleMode,
    } from "./StatusUpdate";
    import Icon from "@iconify/svelte";
    import numeric1BoxOutline from "@iconify/icons-mdi/numeric-1-box-outline";
    import pauseIcon from "@iconify/icons-mdi/pause";
    import playIcon from "@iconify/icons-mdi/play";
    import repeatIcon from "@iconify/icons-mdi/repeat";
    import skipNext from "@iconify/icons-mdi/skip-next";
    import skipPrevious from "@iconify/icons-mdi/skip-previous";
    import TinyInfo from "./TinyInfo.svelte";
    import format from "format-duration";
    import { next, pause, previous, repeat, single } from "./bindings";

    const height = "1.5em";

    let currentProgress = 0;
    let currentTimeStr = "0:00";
    let totalTimeStr = "0:00";

    $: isSingle = $singleMode === "Enabled";
    $: totalTimeStr = format($totalTime);
    $: currentTimeStr = format($currentTime);
    $: currentProgress = calProgress($currentTime, $totalTime);

    function calProgress(now: number, tot: number) {
        if (tot !== 0) {
            return (now / tot) * 100;
        }
        return 0;
    }
    function handlePlay() {
        pause($isPlaying).catch((err) => console.error(err));
    }
    function handleNext() {
        next().catch((err) => console.error(err));
    }
    function handlePrevious() {
        previous().catch((err) => console.error(err));
    }
    function handleRepeat() {
        console.log("Toggle repeat.");
        repeat(!$isRepeat).catch((err) => console.error(err));
    }
    function handleSingle() {
        if (isSingle) {
            console.log("Disable single.");
            single("Disabled").catch((err) => console.error(err));
        } else {
            console.log("Enable single.");
            single("Enabled").catch((err) => console.error(err));
        }
    }
</script>

<div
    class="shrink-0 navbar bg-crust flex flex-col w-auto space-y-0 m-4 rounded-md z-20"
>
    <progress
        class="progress progress-primary"
        value={currentProgress}
        max="100"
    />
    <div class="grow-0 flex flex-row justify-between w-full">
        <p>{currentTimeStr}</p>
        <p>{totalTimeStr}</p>
    </div>
    <div class="flex flex-row w-full">
        <div class="navbar-start">
            <TinyInfo
                title={$currentTitle}
                artist={$currentArtist}
                album={$currentAlbum}
            />
        </div>
        <div class="navbar-center">
            <button class="btn btn-sm btn-ghost" on:click={handleSingle}>
                <div class:text-surface1={!isSingle}>
                    <Icon icon={numeric1BoxOutline} {height} />
                </div>
            </button>
            <div class="join mx-2">
                <button
                    class="btn btn-sm btn-primary join-item"
                    on:click={handlePrevious}
                >
                    <Icon icon={skipPrevious} {height} />
                </button>
                <button
                    class="btn btn-sm btn-primary join-item"
                    on:click={handlePlay}
                >
                    {#if $isPlaying}
                        <Icon icon={pauseIcon} {height} />
                    {:else}
                        <Icon icon={playIcon} {height} />
                    {/if}
                </button>
                <button
                    class="btn btn-sm btn-primary join-item"
                    on:click={handleNext}
                >
                    <Icon icon={skipNext} {height} />
                </button>
            </div>
            <button class="btn btn-sm btn-ghost" on:click={handleRepeat}>
                <div class:text-surface1={!$isRepeat}>
                    <Icon icon={repeatIcon} {height} />
                </div>
            </button>
        </div>
        <div class="navbar-end text-end">{$currentItem}/{$totalItems}</div>
    </div>
</div>
