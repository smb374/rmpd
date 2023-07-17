<script lang="ts">
    export let title: string;
    export let artist: string;
    export let album: string;
</script>

<div class="flex flex-row space-x-2 items-center">
    <div class="flex flex-col">
        <div class="text-sm font-bold">{title}</div>
        <div class="text-sm">{artist}</div>
        <div class="text-xs">{album}</div>
    </div>
</div>
