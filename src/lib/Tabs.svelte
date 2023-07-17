<script lang="ts">
    export let items = [];
    let activeTabValue = 1;

    const labelComponentProps = {
        size: "1.25em",
    };

    const handleActivated = (value: number) => () => (activeTabValue = value);
</script>

<div class="flex-1 flex flex-row justify-items-center space-x-8">
    <div class="flex-none">
        <ul class="flex flex-col bg-transparent space-y-2 p-4">
            {#each items as item}
                <li>
                    {#if activeTabValue == item.value}
                        <button
                            class="btn btn-sm btn-primary"
                            on:click={handleActivated(item.value)}
                        >
                            <svelte:component
                                this={item.labelComponent}
                                {...labelComponentProps}
                            />
                        </button>
                    {:else}
                        <button
                            class="btn btn-sm"
                            on:click={handleActivated(item.value)}
                        >
                            <svelte:component
                                this={item.labelComponent}
                                {...labelComponentProps}
                            />
                        </button>
                    {/if}
                </li>
            {/each}
        </ul>
    </div>
    <div class="flex-1 flex flex-row space-x-8 items-center">
        {#each items as item}
            {#if activeTabValue == item.value}
                <svelte:component
                    this={item.component}
                    {...item.componentProps}
                />
            {/if}
        {/each}
    </div>
</div>
