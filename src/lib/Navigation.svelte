<script lang="ts">
    import PlayerControl from "./PlayerControl.svelte";
    import type { Component } from "./Types";
    import Icon, { type IconifyIcon } from "@iconify/svelte";
    import autoAnimate from "@formkit/auto-animate";

    export let triggers: IconifyIcon[];
    export let contents: Component[];
    let activeTabValue = 0;

    const handleActivated = (value: number) => () => (activeTabValue = value);
</script>

<div class="flex-1 flex flex-row justify-items-center space-x-8">
    <div class="flex-none">
        <ul class="flex flex-col bg-transparent space-y-2 p-4">
            {#each triggers as trigger, idx}
                <li class="transition ease-in-out hover:scale-110">
                    <button
                        class="btn btn-sm"
                        class:btn-primary={activeTabValue === idx}
                        on:click={handleActivated(idx)}
                        ><Icon icon={trigger} height="1.5em" />
                    </button>
                </li>
            {/each}
        </ul>
    </div>
    <div
        class="flex-1 flex flex-row space-x-4 items-center overflow-scroll"
        use:autoAnimate
    >
        {#each contents as content, idx}
            {#if activeTabValue == idx}
                <svelte:component this={content.component} {...content.props} />
            {/if}
        {/each}
    </div>
</div>
<PlayerControl />
