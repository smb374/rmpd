/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function getCoverPath(url: string) {
    return invoke()<string>("get_cover_path", { url })
}

export function add(uri: string, prop: AddVariant) {
    return invoke()<number>("add", { uri,prop })
}

export function playlistadd(playlist: string, uri: string, position: number | null) {
    return invoke()<null>("playlistadd", { playlist,uri,position })
}

export function albumart(uri: string, offset: number) {
    return invoke()<AlbumArt | null>("albumart", { uri,offset })
}

export function readpicture(uri: string, offset: number) {
    return invoke()<AlbumArt | null>("readpicture", { uri,offset })
}

export function playlistclear(playlist: string) {
    return invoke()<null>("playlistclear", { playlist })
}

export function clear() {
    return invoke()<null>("clear")
}

export function count(filter: Filter) {
    return invoke()<Count | null>("count", { filter })
}

export function countByGroup(group: Tag, filter: Filter | null) {
    return invoke()<([string, Count])[]>("count_by_group", { group,filter })
}

export function crossfade(duration: TVal) {
    return invoke()<null>("crossfade", { duration })
}

export function currentsong() {
    return invoke()<SongInQueue | null>("currentsong")
}

export function deleteItem(variant: DeleteVariant) {
    return invoke()<null>("delete_item", { variant })
}

export function rm(playlist: string) {
    return invoke()<null>("rm", { playlist })
}

export function find(filter: Filter, sort: Tag | null) {
    return invoke()<Song[]>("find", { filter,sort })
}

export function tagtypes() {
    return invoke()<Tag[]>("tagtypes")
}

export function listplaylistinfo(playlist: string) {
    return invoke()<Song[]>("listplaylistinfo", { playlist })
}

export function listplaylists() {
    return invoke()<Playlist[]>("listplaylists")
}

export function list(tag: Tag, filter: Filter | null) {
    return invoke()<List>("list", { tag,filter })
}

export function listallinfo(directory: string | null) {
    return invoke()<Song[]>("listallinfo", { directory })
}

export function channels() {
    return invoke()<string[]>("channels")
}

export function load(playlist: string, range: [number, number] | null) {
    return invoke()<null>("load", { playlist,range })
}

export function moveSong(variant: MoveVariant, target: MoveTarget) {
    return invoke()<null>("move_song", { variant,target })
}

export function next() {
    return invoke()<null>("next")
}

export function ping() {
    return invoke()<null>("ping")
}

export function play(variant: ItemVariant | null) {
    return invoke()<null>("play", { variant })
}

export function previous() {
    return invoke()<null>("previous")
}

export function playlistinfo() {
    return invoke()<SongInQueue[]>("playlistinfo")
}

export function playlistdelete(playlist: string, variant: PlaylistDeleteVariant) {
    return invoke()<null>("playlistdelete", { playlist,variant })
}

export function rename(from: string, to: string) {
    return invoke()<null>("rename", { from,to })
}

export function rescan(uri: string | null) {
    return invoke()<number>("rescan", { uri })
}

export function save(playlist: string) {
    return invoke()<null>("save", { playlist })
}

export function seekcur(seekMode: SeekMode) {
    return invoke()<null>("seekcur", { seekMode })
}

export function seek(variant: ItemVariant, position: TVal) {
    return invoke()<null>("seek", { variant,position })
}

export function consume(consume: boolean) {
    return invoke()<null>("consume", { consume })
}

export function pause(pause: boolean) {
    return invoke()<null>("pause", { pause })
}

export function random(random: boolean) {
    return invoke()<null>("random", { random })
}

export function single(single: SingleMode) {
    return invoke()<null>("single", { single })
}

export function repeat(repeat: boolean) {
    return invoke()<null>("repeat", { repeat })
}

export function setvol(volume: number) {
    return invoke()<null>("setvol", { volume })
}

export function stats() {
    return invoke()<Stats>("stats")
}

export function status() {
    return invoke()<Status>("status")
}

export function stickerDelete(uri: string, name: string) {
    return invoke()<null>("sticker_delete", { uri,name })
}

export function stickerFind(uri: string, name: string, value: string, variant: StickerFindVariant) {
    return invoke()<StickerFind>("sticker_find", { uri,name,value,variant })
}

export function stickerGet(uri: string, name: string) {
    return invoke()<StickerGet>("sticker_get", { uri,name })
}

export function stickerList(uri: string) {
    return invoke()<StickerList>("sticker_list", { uri })
}

export function stickerSet(uri: string, name: string, value: string) {
    return invoke()<null>("sticker_set", { uri,name,value })
}

export function stop() {
    return invoke()<null>("stop")
}

export function subscribe(channel: string) {
    return invoke()<null>("subscribe", { channel })
}

export function unsubscribe(channel: string) {
    return invoke()<null>("unsubscribe", { channel })
}

export function update(uri: string | null) {
    return invoke()<number>("update", { uri })
}

export type StickerFindVariant = "Eq" | "Gt" | "Lt"
export type SingleMode = "Enabled" | "Disabled" | "Oneshot"
export type Song = { url: string; duration: TVal | null; tags: { [key: string]: string[] }; format: string | null; last_modified: Timestamp | null }
export type Filter = { elements: FilterElement[]; negate: boolean }
export type MoveVariant = { Id: number } | { Position: number } | { Range: [number, number] }
export type Count = { songs: number; playtime: TVal }
export type SeekMode = { Forward: TVal } | { Backward: TVal } | { Absolute: TVal }
export type SongInQueue = { position: number; id: number; range: SongRange | null; priority: number; song: Song }
export type StickerList = { value: { [key: string]: string } }
export type List = { [key: string]: string[] }
export type ItemVariant = { Id: number } | { Position: number }
export type Stats = { artists: number; albums: number; songs: number; uptime: TVal; playtime: TVal; db_playtime: TVal; db_last_update: number }
export type StickerGet = { value: string }
export type AddVariant = "Append" | { At: number } | { BeforeCurrent: number } | { AfterCurrent: number }
export type Timestamp = string
export type Playlist = { name: string; last_modified: Timestamp }
export type MoveTarget = { Position: number } | { BeforeCurrent: number } | { AfterCurrent: number }
export type AlbumArt = { size: number; mime: string | null; data: number[] }
export type StickerFind = { value: { [key: string]: string } }
export type FilterElement = { tag: Tag; variant: FilterVariant; negate: boolean }
export type DeleteVariant = { Id: number } | { Position: number } | { Range: [number, number] }
export type SongRange = { from: TVal; to: TVal | null }
export type PlaylistDeleteVariant = { Position: number } | { Range: [number, number] }
export type FilterVariant = { Expr: { op: Operator; val: string } } | "Exist" | "Absent"
export type Status = { volume: number; state: PlayState; repeat: boolean; random: boolean; consume: boolean; single: SingleMode; playlist_version: number; playlist_length: number; current_song: [number, number] | null; next_song: [number, number] | null; elapsed: TVal | null; duration: TVal | null; bitrate: number | null; crossfade: TVal; update_job: number | null; error: string | null; partition: string | null }
export type Tag = "Album" | "AlbumArtist" | "AlbumArtistSort" | "AlbumSort" | "Artist" | "ArtistSort" | "Comment" | "Composer" | "ComposerSort" | "Conductor" | "Date" | "Disc" | "Ensemble" | "Genre" | "Grouping" | "Label" | "Location" | "Movement" | "MovementNumber" | "MusicBrainzArtistId" | "MusicBrainzRecordingId" | "MusicBrainzReleaseArtistId" | "MusicBrainzReleaseId" | "MusicBrainzTrackId" | "MusicBrainzWorkId" | "Name" | "OriginalDate" | "Performer" | "Title" | "Track" | "Work" | { Other: string }
export type TVal = { secs: number; nanos: number }
export type Operator = "Equal" | "NotEqual" | "Contain" | "Match" | "NotMatch"
export type PlayState = "Stopped" | "Playing" | "Paused"