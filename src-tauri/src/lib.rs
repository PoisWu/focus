mod commands;
mod error;
mod models;

use commands::{photo_cache, spotify};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();

    let log_level = std::env::var("LOG_LEVEL")
        .ok()
        .and_then(|v| v.parse::<log::LevelFilter>().ok())
        .unwrap_or(log::LevelFilter::Debug);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("focus".into()),
                    }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .level(log_level)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            photo_cache::fetch_photos,
            photo_cache::get_cached_photos,
            spotify::spotify_play_pause,
            spotify::spotify_next,
            spotify::spotify_get_track
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
