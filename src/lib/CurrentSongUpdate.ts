import { writable } from "svelte/store";
import { getCoverPath, type SongInQueue } from "./bindings";

const coverPath = writable("/home/poyehchen/placeholder.png");
const currentItem = writable(0);
const currentTitle = writable("Unknown Title");
const currentArtist = writable("Unknown Artist");
const currentAlbum = writable("Unknown Album");

function currentSongUpdate(currentSong: SongInQueue) {
    currentItem.set(currentSong.position);
    const song = currentSong.song;
    const tags = song.tags;
    getCoverPath(song.url).then((path: string) => coverPath.set(path));
    currentTitle.update((orig) => tags?.["Title"]?.[0] || orig);
    currentArtist.update((orig) => tags?.["Artist"]?.[0] || orig);
    currentAlbum.update((orig) => tags?.["Album"]?.[0] || orig);
}

export {
    coverPath,
    currentItem,
    currentTitle,
    currentArtist,
    currentAlbum,
    currentSongUpdate,
};
