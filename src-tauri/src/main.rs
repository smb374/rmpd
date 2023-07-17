// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod ipc;
mod mpd;
mod reflection;

use ipc::*;
use mpd::{event_handler, initialize_connection};
use tauri::{async_runtime, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let (client, events) =
                async_runtime::block_on(initialize_connection("localhost:6600")).expect("Failed");
            app.manage(client);
            async_runtime::spawn(event_handler(handle, events));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // sync
            get_cover_path,
            // mpd
            add,
            playlistadd,
            albumart,
            readpicture,
            playlistclear,
            clear,
            count,
            count_by_group,
            crossfade,
            currentsong,
            delete_item,
            rm,
            find,
            tagtypes,
            listplaylistinfo,
            listplaylists,
            list,
            listallinfo,
            channels,
            load,
            move_song,
            next,
            ping,
            play,
            previous,
            playlistinfo,
            playlistdelete,
            rename,
            rescan,
            save,
            seekcur,
            seek,
            consume,
            pause,
            random,
            single,
            repeat,
            setvol,
            stats,
            status,
            sticker_delete,
            sticker_find,
            sticker_get,
            sticker_list,
            sticker_set,
            stop,
            subscribe,
            unsubscribe,
            update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::ipc::*;
    use specta::{
        collect_types,
        ts::{BigIntExportBehavior, ExportConfiguration},
    };
    use tauri_specta::ts::export_with_cfg;
    #[test]
    fn export_type_bindings() {
        let list = collect_types![
            // sync
            get_cover_path,
            // mpd
            add,
            playlistadd,
            albumart,
            readpicture,
            playlistclear,
            clear,
            count,
            count_by_group,
            crossfade,
            currentsong,
            delete_item,
            rm,
            find,
            tagtypes,
            listplaylistinfo,
            listplaylists,
            list,
            listallinfo,
            channels,
            load,
            move_song,
            next,
            ping,
            play,
            previous,
            playlistinfo,
            playlistdelete,
            rename,
            rescan,
            save,
            seekcur,
            seek,
            consume,
            pause,
            random,
            single,
            repeat,
            setvol,
            stats,
            status,
            sticker_delete,
            sticker_find,
            sticker_get,
            sticker_list,
            sticker_set,
            stop,
            subscribe,
            unsubscribe,
            update,
        ]
        .unwrap();
        export_with_cfg(
            list,
            ExportConfiguration::default().bigint(BigIntExportBehavior::Number),
            "../src/lib/bindings.ts",
        )
        .unwrap();
    }
}
