<script lang="ts">
    import { currentItem } from "./CurrentSongUpdate";
    import { currentQueue } from "./GlobalStates";
    import { tvalToMs } from "./Utils";
    import type { SongInQueue } from "./bindings";
    import formatDuration from "format-duration";
    import Icon from "@iconify/svelte";
    import chevronRight from "@iconify/icons-mdi/chevron-right";
    import chevronLeft from "@iconify/icons-mdi/chevron-left";
    import { totalItems } from "./StatusUpdate";
    import { play } from "./bindings";

    const height = "1.5em";
    const maxPage = Math.floor($totalItems / 10);

    // 10 rows per page
    let queueDisplay: SongInQueue[] = [];
    let page = 0;
    $: page = Math.floor($currentItem / 10);
    $: start = page * 10;
    $: end = (page + 1) * 10;
    $: queueDisplay = $currentQueue.slice(start, end);

    function handlePreviousPage() {
        if (page - 1 >= 0) {
            page = page - 1;
        }
    }
    function handleNextPage() {
        if (page + 1 < maxPage) {
            page = page + 1;
        }
    }
    async function handleItemClick(id: number) {
        await play({ Id: id });
    }
</script>

<div class="flex-1 mx-4 flex flex-col items-center space-y-4">
    <div class="flex flex-col space-y-2 w-full">
        {#each queueDisplay as item}
            <button
                class="flex flex-row text-start space-x-4 bg-crust rounded-md py-2 px-4"
                class:bg-primary={$currentItem === item.position}
                class:text-base={$currentItem === item.position}
                on:click={() => handleItemClick(item.id)}
            >
                <div class="flex-none w-16">{item.position + 1}</div>
                <div class="flex-1">
                    {(item.song.tags?.["Album"]?.[0] || "Unknown") +
                        ": " +
                        (item.song.tags?.["Title"]?.[0] || "Unknown")}
                </div>
                <div class="flex-none">
                    {formatDuration(tvalToMs(item.song.duration))}
                </div>
            </button>
        {/each}
    </div>
    <div class="join">
        <button class="join-item btn" on:click={handlePreviousPage}>
            <Icon icon={chevronLeft} {height} />
        </button>
        <div class="flex join-item bg-crust px-4">
            <span class="self-center font-bold">{page + 1}</span>
        </div>
        <button class="join-item btn" on:click={handleNextPage}>
            <Icon icon={chevronRight} {height} />
        </button>
    </div>
</div>
