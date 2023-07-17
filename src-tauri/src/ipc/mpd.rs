use std::time::Duration;

use mpd_client::{
    commands::{self, SongId, SongPosition},
    filter,
    tag::TagError,
    Client,
};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

use crate::reflection::{
    request::{SeekMode, SingleMode},
    response::{
        AlbumArt, Count, List, Playlist, Song, SongInQueue, Stats, Status, StickerFind, StickerGet,
        StickerList, TVal,
    },
    tag::Tag,
    Reflect,
};

#[derive(Serialize, Deserialize, Type)]
pub enum AddVariant {
    Append,
    At(usize),
    BeforeCurrent(usize),
    AfterCurrent(usize),
}

#[derive(Serialize, Deserialize, Type)]
pub enum Operator {
    Equal,
    NotEqual,
    Contain,
    Match,
    NotMatch,
}

impl From<Operator> for filter::Operator {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Equal => filter::Operator::Equal,
            Operator::NotEqual => filter::Operator::NotEqual,
            Operator::Contain => filter::Operator::Contain,
            Operator::Match => filter::Operator::Match,
            Operator::NotMatch => filter::Operator::NotMatch,
        }
    }
}

#[derive(Serialize, Deserialize, Type)]
pub enum FilterVariant {
    Expr { op: Operator, val: String },
    Exist,
    Absent,
}

#[derive(Serialize, Deserialize, Type)]
pub struct FilterElement {
    tag: Tag,
    variant: FilterVariant,
    negate: bool,
}

#[derive(Serialize, Deserialize, Type)]
pub struct Filter {
    elements: Vec<FilterElement>,
    negate: bool,
}

impl Filter {
    fn render(self) -> Result<Option<filter::Filter>, TagError> {
        if self.elements.is_empty() {
            Ok(None)
        } else {
            let mut result =
                self.elements
                    .into_iter()
                    .try_fold(None, |acc: Option<filter::Filter>, elem| {
                        let tag = mpd_client::tag::Tag::try_from(elem.tag.to_string().as_str())?;
                        let mut f = match elem.variant {
                            FilterVariant::Expr { op, val } => {
                                filter::Filter::new(tag, op.into(), val)
                            }
                            FilterVariant::Exist => filter::Filter::tag_exists(tag),
                            FilterVariant::Absent => filter::Filter::tag_absent(tag),
                        };
                        if elem.negate {
                            f = f.negate();
                        }
                        match acc {
                            Some(acc) => Ok(Some(acc.and(f))),
                            None => Ok(Some(f)),
                        }
                    })?;
            if self.negate {
                result = result.map(|f| f.negate());
            }
            Ok(result)
        }
    }
}

#[derive(Serialize, Deserialize, Type)]
pub enum DeleteVariant {
    Id(u64),
    Position(usize),
    Range(usize, usize),
}

#[derive(Serialize, Deserialize, Type)]
pub enum MoveVariant {
    Id(u64),
    Position(usize),
    Range(usize, usize),
}

#[derive(Serialize, Deserialize, Type)]
pub enum MoveTarget {
    Position(usize),
    BeforeCurrent(usize),
    AfterCurrent(usize),
}

#[derive(Serialize, Deserialize, Type)]
pub enum PlaylistDeleteVariant {
    Position(usize),
    Range(usize, usize),
}

#[derive(Serialize, Deserialize, Type)]
pub enum ItemVariant {
    Id(u64),
    Position(usize),
}

impl From<ItemVariant> for commands::Song {
    fn from(value: ItemVariant) -> Self {
        match value {
            ItemVariant::Id(id) => commands::Song::Id(SongId(id)),
            ItemVariant::Position(pos) => commands::Song::Position(SongPosition(pos)),
        }
    }
}

#[derive(Serialize, Deserialize, Type)]
pub enum StickerFindVariant {
    Eq,
    Gt,
    Lt,
}

#[tauri::command]
#[specta::specta]
pub async fn add(client: State<'_, Client>, uri: String, prop: AddVariant) -> Result<u64, String> {
    let mut cmd = commands::Add::uri(&uri);
    cmd = match prop {
        AddVariant::Append => cmd,
        AddVariant::At(pos) => cmd.at(pos),
        AddVariant::BeforeCurrent(delta) => cmd.before_current(delta),
        AddVariant::AfterCurrent(delta) => cmd.after_current(delta),
    };
    let res = client
        .command(cmd)
        .await
        .map(|x| x.0)
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn playlistadd(
    client: State<'_, Client>,
    playlist: String,
    uri: String,
    position: Option<usize>,
) -> Result<(), String> {
    let mut cmd = commands::AddToPlaylist::new(&playlist, &uri);
    if let Some(pos) = position {
        cmd = cmd.at(pos);
    }
    client.command(cmd).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn albumart(
    client: State<'_, Client>,
    uri: String,
    offset: usize,
) -> Result<Option<AlbumArt>, String> {
    let cmd = commands::AlbumArt::new(&uri).offset(offset);
    let res = client
        .command(cmd)
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn readpicture(
    client: State<'_, Client>,
    uri: String,
    offset: usize,
) -> Result<Option<AlbumArt>, String> {
    let cmd = commands::AlbumArtEmbedded::new(&uri).offset(offset);
    let res = client
        .command(cmd)
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn playlistclear(client: State<'_, Client>, playlist: String) -> Result<(), String> {
    let cmd = commands::ClearPlaylist(&playlist);
    client.command(cmd).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn clear(client: State<'_, Client>) -> Result<(), String> {
    client
        .command(commands::ClearQueue)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn count(client: State<'_, Client>, filter: Filter) -> Result<Option<Count>, String> {
    if let Some(filter) = filter.render().map_err(|e| e.to_string())? {
        let cmd = commands::Count::new(filter);
        let res = client
            .command(cmd)
            .await
            .map(|x| Some(x.reflect()))
            .map_err(|e| e.to_string())?;
        Ok(res)
    } else {
        Ok(None)
    }
}

#[tauri::command]
#[specta::specta]
pub async fn count_by_group(
    client: State<'_, Client>,
    group: Tag,
    filter: Option<Filter>,
) -> Result<Vec<(String, Count)>, String> {
    let mut cmd = commands::CountGrouped::new(group.render().map_err(|e| e.to_string())?);
    if let Some(filter) = filter {
        if let Some(f) = filter.render().map_err(|e| e.to_string())? {
            cmd = cmd.filter(f);
        }
    }
    let resp = client.command(cmd).await.map_err(|e| e.to_string())?;
    Ok(resp.into_iter().map(|(t, c)| (t, c.reflect())).collect())
}

#[tauri::command]
#[specta::specta]
pub async fn crossfade(client: State<'_, Client>, duration: TVal) -> Result<(), String> {
    let dur = Duration::new(duration.secs, duration.nanos);
    client
        .command(commands::Crossfade(dur))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn currentsong(client: State<'_, Client>) -> Result<Option<SongInQueue>, String> {
    let res = client
        .command(commands::CurrentSong)
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_item(client: State<'_, Client>, variant: DeleteVariant) -> Result<(), String> {
    let cmd = match variant {
        DeleteVariant::Id(id) => commands::Delete::id(SongId(id)),
        DeleteVariant::Position(pos) => commands::Delete::position(SongPosition(pos)),
        DeleteVariant::Range(start, end) => {
            commands::Delete::range(SongPosition(start)..SongPosition(end))
        }
    };
    client.command(cmd).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn rm(client: State<'_, Client>, playlist: String) -> Result<(), String> {
    client
        .command(commands::DeletePlaylist(&playlist))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn find(
    client: State<'_, Client>,
    filter: Filter,
    sort: Option<Tag>,
) -> Result<Vec<Song>, String> {
    let filter = filter.render().map_err(|e| e.to_string())?;
    if let Some(f) = filter {
        let mut cmd = commands::Find::new(f);
        if let Some(tag) = sort {
            let t = tag.render().map_err(|e| e.to_string())?;
            cmd = cmd.sort(t);
        }
        let res = client
            .command(cmd)
            .await
            .map(|x| x.reflect())
            .map_err(|e| e.to_string())?;
        Ok(res)
    } else {
        Err("Empty filter.".to_string())
    }
}

#[tauri::command]
#[specta::specta]
pub async fn tagtypes(client: State<'_, Client>) -> Result<Vec<Tag>, String> {
    let res = client
        .command(commands::GetEnabledTagTypes)
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn listplaylistinfo(
    client: State<'_, Client>,
    playlist: String,
) -> Result<Vec<Song>, String> {
    let res = client
        .command(commands::GetPlaylist(&playlist))
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn listplaylists(client: State<'_, Client>) -> Result<Vec<Playlist>, String> {
    let res = client
        .command(commands::GetPlaylists)
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn list(
    client: State<'_, Client>,
    tag: Tag,
    filter: Option<Filter>,
) -> Result<List, String> {
    let mut cmd = commands::List::new(tag.render().map_err(|e| e.to_string())?);
    if let Some(filter) = filter {
        if let Some(f) = filter.render().map_err(|e| e.to_string())? {
            cmd = cmd.filter(f);
        }
    }
    let res = client
        .command(cmd)
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

// #[tauri::command]
// #[specta::specta]
// pub async fn list_by_group(
//     client: State<'_, Client>,
//     tag: Tag,
//     group: Vec<Tag>,
//     filter: Option<Filter>,
// ) -> Result<List, String> {
//     let cmd = commands::List::new(tag.render().map_err(|e| e.to_string())?);
//     let group = array![[group; group.len()]];
//     let mut cmd: commands::List = cmd.group_by(group);
//     if let Some(filter) = filter {
//         if let Some(f) = filter.render().map_err(|e| e.to_string())? {
//             cmd = cmd.filter(f);
//         }
//     }
//     let res = client
//         .command(cmd)
//         .await
//         .map(|x| x.reflect())
//         .map_err(|e| e.to_string())?;
//     Ok(res)
// }
//
#[tauri::command]
#[specta::specta]
pub async fn listallinfo(
    client: State<'_, Client>,
    directory: Option<String>,
) -> Result<Vec<Song>, String> {
    if let Some(dir) = directory {
        let cmd = commands::ListAllIn::directory(&dir);
        let res = client
            .command(cmd)
            .await
            .map(|x| x.reflect())
            .map_err(|e| e.to_string())?;
        Ok(res)
    } else {
        let cmd = commands::ListAllIn::root();
        let res = client
            .command(cmd)
            .await
            .map(|x| x.reflect())
            .map_err(|e| e.to_string())?;
        Ok(res)
    }
}

#[tauri::command]
#[specta::specta]
pub async fn channels(client: State<'_, Client>) -> Result<Vec<String>, String> {
    let res = client
        .command(commands::ListChannels)
        .await
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn load(
    client: State<'_, Client>,
    playlist: String,
    range: Option<(usize, usize)>,
) -> Result<(), String> {
    let mut cmd = commands::LoadPlaylist::name(&playlist);
    if let Some((start, end)) = range {
        cmd = cmd.range(start..end);
    }
    client.command(cmd).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn move_song(
    client: State<'_, Client>,
    variant: MoveVariant,
    target: MoveTarget,
) -> Result<(), String> {
    let cmd_builder = match variant {
        MoveVariant::Id(id) => commands::Move::id(SongId(id)),
        MoveVariant::Position(pos) => mpd_client::commands::Move::position(SongPosition(pos)),
        MoveVariant::Range(start, end) => {
            commands::Move::range(SongPosition(start)..SongPosition(end))
        }
    };
    let cmd = match target {
        MoveTarget::Position(pos) => cmd_builder.to_position(SongPosition(pos)),
        MoveTarget::AfterCurrent(delta) => cmd_builder.after_current(delta),
        MoveTarget::BeforeCurrent(delta) => cmd_builder.before_current(delta),
    };
    client.command(cmd).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn next(client: State<'_, Client>) -> Result<(), String> {
    client
        .command(commands::Next)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn ping(client: State<'_, Client>) -> Result<(), String> {
    client
        .command(commands::Ping)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn play(client: State<'_, Client>, variant: Option<ItemVariant>) -> Result<(), String> {
    let cmd = if let Some(v) = variant {
        commands::Play::song(commands::Song::from(v))
    } else {
        commands::Play::current()
    };
    client.command(cmd).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn previous(client: State<'_, Client>) -> Result<(), String> {
    client
        .command(commands::Previous)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn playlistinfo(client: State<'_, Client>) -> Result<Vec<SongInQueue>, String> {
    let res = client
        .command(commands::Queue)
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn playlistdelete(
    client: State<'_, Client>,
    playlist: String,
    variant: PlaylistDeleteVariant,
) -> Result<(), String> {
    let cmd = match variant {
        PlaylistDeleteVariant::Position(pos) => {
            commands::RemoveFromPlaylist::position(&playlist, pos)
        }
        PlaylistDeleteVariant::Range(start, end) => {
            commands::RemoveFromPlaylist::range(&playlist, SongPosition(start)..SongPosition(end))
        }
    };
    client.command(cmd).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn rename(client: State<'_, Client>, from: String, to: String) -> Result<(), String> {
    client
        .command(commands::RenamePlaylist::new(&from, &to))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn rescan(client: State<'_, Client>, uri: Option<String>) -> Result<u64, String> {
    let res = if let Some(uri) = uri {
        let cmd = commands::Rescan::new().uri(&uri);
        client.command(cmd).await.map_err(|e| e.to_string())?
    } else {
        let cmd = commands::Rescan::new();
        client.command(cmd).await.map_err(|e| e.to_string())?
    };
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn save(client: State<'_, Client>, playlist: String) -> Result<(), String> {
    client
        .command(commands::SaveQueueAsPlaylist(&playlist))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn seekcur(client: State<'_, Client>, seek_mode: SeekMode) -> Result<(), String> {
    client
        .command(commands::Seek(seek_mode.into()))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn seek(
    client: State<'_, Client>,
    variant: ItemVariant,
    position: TVal,
) -> Result<(), String> {
    client
        .command(commands::SeekTo(variant.into(), position.into()))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn consume(client: State<'_, Client>, consume: bool) -> Result<(), String> {
    client
        .command(commands::SetConsume(consume))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pause(client: State<'_, Client>, pause: bool) -> Result<(), String> {
    client
        .command(commands::SetPause(pause))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn random(client: State<'_, Client>, random: bool) -> Result<(), String> {
    client
        .command(commands::SetRandom(random))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn repeat(client: State<'_, Client>, repeat: bool) -> Result<(), String> {
    client
        .command(commands::SetRepeat(repeat))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn single(client: State<'_, Client>, single: SingleMode) -> Result<(), String> {
    client
        .command(commands::SetSingle(single.into()))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn setvol(client: State<'_, Client>, volume: u8) -> Result<(), String> {
    client
        .command(commands::SetVolume(volume))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn stats(client: State<'_, Client>) -> Result<Stats, String> {
    let res = client
        .command(commands::Stats)
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn status(client: State<'_, Client>) -> Result<Status, String> {
    let res = client
        .command(commands::Status)
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn sticker_delete(
    client: State<'_, Client>,
    uri: String,
    name: String,
) -> Result<(), String> {
    client
        .command(commands::StickerDelete::new(&uri, &name))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn sticker_find(
    client: State<'_, Client>,
    uri: String,
    name: String,
    value: String,
    variant: StickerFindVariant,
) -> Result<StickerFind, String> {
    let cmd = match variant {
        StickerFindVariant::Eq => commands::StickerFind::new(&uri, &name).where_eq(&value),
        StickerFindVariant::Gt => commands::StickerFind::new(&uri, &name).where_gt(&value),
        StickerFindVariant::Lt => commands::StickerFind::new(&uri, &name).where_lt(&value),
    };
    let res = client
        .command(cmd)
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn sticker_get(
    client: State<'_, Client>,
    uri: String,
    name: String,
) -> Result<StickerGet, String> {
    let res = client
        .command(commands::StickerGet::new(&uri, &name))
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn sticker_list(client: State<'_, Client>, uri: String) -> Result<StickerList, String> {
    let res = client
        .command(commands::StickerList::new(&uri))
        .await
        .map(|x| x.reflect())
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub async fn sticker_set(
    client: State<'_, Client>,
    uri: String,
    name: String,
    value: String,
) -> Result<(), String> {
    client
        .command(commands::StickerSet::new(&uri, &name, &value))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn stop(client: State<'_, Client>) -> Result<(), String> {
    client
        .command(commands::Stop)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn subscribe(client: State<'_, Client>, channel: String) -> Result<(), String> {
    client
        .command(commands::SubscribeToChannel(&channel))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn unsubscribe(client: State<'_, Client>, channel: String) -> Result<(), String> {
    client
        .command(commands::UnsubscribeFromChannel(&channel))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn update(client: State<'_, Client>, uri: Option<String>) -> Result<u64, String> {
    let res = match uri {
        Some(uri) => client
            .command(commands::Update::new().uri(&uri))
            .await
            .map_err(|e| e.to_string())?,
        None => client
            .command(commands::Update::new())
            .await
            .map_err(|e| e.to_string())?,
    };
    Ok(res)
}
