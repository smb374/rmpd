use std::{fs, path::PathBuf};

#[tauri::command]
#[specta::specta]
pub fn get_cover_path(url: String) -> String {
    let path = PathBuf::from(format!("/home/poyehchen/Music/{}", url));
    let dir = path.parent().unwrap().to_string_lossy();
    let extensions = ["png", "jpg", "jpeg"];
    for ext in extensions {
        let p: String = format!("{}/cover.{}", dir, ext);
        if fs::metadata(&p).is_ok() {
            return p;
        }
    }
    String::from("/home/poyehchen/placeholder.png")
}
