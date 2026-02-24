mod commands;
mod error;
mod models;

use commands::{photo_cache, photos, spotify};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            photos::fetch_photos,
            photo_cache::get_cached_photos,
            photo_cache::fetch_and_cache_photos,
            spotify::spotify_play_pause,
            spotify::spotify_next,
            spotify::spotify_get_track
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
