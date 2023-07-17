use std::{collections::HashMap, time::Duration, usize};

use bytes::BytesMut;
use mpd_client::responses;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::{request::SingleMode, Reflect};

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct TVal {
    pub secs: u64,
    pub nanos: u32,
}

impl Reflect for Duration {
    type Output = TVal;
    fn reflect(self) -> Self::Output {
        TVal {
            secs: self.as_secs(),
            nanos: self.subsec_nanos(),
        }
    }
}

impl From<TVal> for Duration {
    fn from(value: TVal) -> Self {
        Duration::new(value.secs, value.nanos)
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct AlbumArt {
    pub size: usize,
    pub mime: Option<String>,
    #[specta(type = Vec<u8>)]
    pub data: BytesMut,
}

impl Reflect for responses::AlbumArt {
    type Output = AlbumArt;
    fn reflect(self) -> Self::Output {
        Self::Output {
            size: self.size,
            mime: self.mime,
            data: self.data,
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct Count {
    pub songs: u64,
    pub playtime: TVal,
}

impl Reflect for responses::Count {
    type Output = Count;
    fn reflect(self) -> Self::Output {
        Self::Output {
            songs: self.songs,
            playtime: self.playtime.reflect(),
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct List(pub HashMap<String, Vec<String>>);

impl<const N: usize> Reflect for responses::List<N> {
    type Output = List;
    fn reflect(self) -> Self::Output {
        let vals = self.into_raw_values();
        let mut map: HashMap<String, Vec<String>> = HashMap::with_capacity(vals.len());
        for (k, v) in vals {
            let tag = k.reflect().to_string();
            if let Some(vec) = map.get_mut(&tag) {
                vec.push(v);
            } else {
                map.insert(tag, vec![v]);
            }
        }
        List(map)
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct Playlist {
    pub name: String,
    pub last_modified: Timestamp,
}

impl Reflect for responses::Playlist {
    type Output = Playlist;
    fn reflect(self) -> Self::Output {
        Self::Output {
            name: self.name,
            last_modified: self.last_modified.reflect(),
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub enum PlayState {
    Stopped,
    Playing,
    Paused,
}

impl Reflect for responses::PlayState {
    type Output = PlayState;
    fn reflect(self) -> Self::Output {
        match self {
            responses::PlayState::Stopped => PlayState::Stopped,
            responses::PlayState::Playing => PlayState::Playing,
            responses::PlayState::Paused => PlayState::Paused,
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct Song {
    pub url: String,
    pub duration: Option<TVal>,
    pub tags: HashMap<String, Vec<String>>,
    pub format: Option<String>,
    pub last_modified: Option<Timestamp>,
}

impl Reflect for responses::Song {
    type Output = Song;
    fn reflect(self) -> Self::Output {
        let tags = self
            .tags
            .into_iter()
            .map(|(k, v)| (k.reflect().to_string(), v))
            .collect();
        Song {
            url: self.url,
            duration: self.duration.reflect(),
            tags,
            format: self.format,
            last_modified: self.last_modified.map(|x| x.reflect()),
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct SongInQueue {
    pub position: usize,
    pub id: u64,
    pub range: Option<SongRange>,
    pub priority: u8,
    pub song: Song,
}

impl Reflect for responses::SongInQueue {
    type Output = SongInQueue;
    fn reflect(self) -> Self::Output {
        SongInQueue {
            position: self.position.0,
            id: self.id.0,
            range: self.range.map(|x| x.reflect()),
            priority: self.priority,
            song: self.song.reflect(),
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct SongRange {
    pub from: TVal,
    pub to: Option<TVal>,
}

impl Reflect for responses::SongRange {
    type Output = SongRange;
    fn reflect(self) -> Self::Output {
        SongRange {
            from: self.from.reflect(),
            to: self.to.reflect(),
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct Stats {
    pub artists: u64,
    pub albums: u64,
    pub songs: u64,
    pub uptime: TVal,
    pub playtime: TVal,
    pub db_playtime: TVal,
    pub db_last_update: u64,
}

impl Reflect for responses::Stats {
    type Output = Stats;
    fn reflect(self) -> Self::Output {
        Stats {
            artists: self.artists,
            albums: self.albums,
            songs: self.songs,
            uptime: self.uptime.reflect(),
            playtime: self.playtime.reflect(),
            db_playtime: self.db_playtime.reflect(),
            db_last_update: self.db_last_update,
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct Status {
    pub volume: u8,
    pub state: PlayState,
    pub repeat: bool,
    pub random: bool,
    pub consume: bool,
    pub single: SingleMode,
    pub playlist_version: u32,
    pub playlist_length: usize,
    pub current_song: Option<(usize, u64)>,
    pub next_song: Option<(usize, u64)>,
    pub elapsed: Option<TVal>,
    pub duration: Option<TVal>,
    pub bitrate: Option<u64>,
    pub crossfade: TVal,
    pub update_job: Option<u64>,
    pub error: Option<String>,
    pub partition: Option<String>,
}

impl Reflect for responses::Status {
    type Output = Status;
    fn reflect(self) -> Self::Output {
        Status {
            volume: self.volume,
            state: self.state.reflect(),
            repeat: self.repeat,
            random: self.random,
            consume: self.consume,
            single: self.single.reflect(),
            playlist_version: self.playlist_version,
            playlist_length: self.playlist_length,
            current_song: self.current_song.map(|(x, y)| (x.0, y.0)),
            next_song: self.current_song.map(|(x, y)| (x.0, y.0)),
            elapsed: self.elapsed.reflect(),
            duration: self.duration.reflect(),
            bitrate: self.bitrate,
            crossfade: self.crossfade.reflect(),
            update_job: self.update_job,
            error: self.error,
            partition: self.partition,
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct StickerFind {
    pub value: HashMap<String, String>,
}

impl Reflect for responses::StickerFind {
    type Output = StickerFind;
    fn reflect(self) -> Self::Output {
        StickerFind { value: self.value }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct StickerGet {
    pub value: String,
}

impl Reflect for responses::StickerGet {
    type Output = StickerGet;
    fn reflect(self) -> Self::Output {
        StickerGet { value: self.value }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct StickerList {
    pub value: HashMap<String, String>,
}

impl Reflect for responses::StickerList {
    type Output = StickerList;
    fn reflect(self) -> Self::Output {
        StickerList { value: self.value }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct Timestamp(pub String);

impl Reflect for responses::Timestamp {
    type Output = Timestamp;
    fn reflect(self) -> Self::Output {
        Timestamp(self.raw().to_string())
    }
}
