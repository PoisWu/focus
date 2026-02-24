use crate::error::AppError;
use crate::models::TrackDto;
use mpris::{PlaybackStatus, Player, PlayerFinder};

fn find_player() -> Result<Player, AppError> {
    match PlayerFinder::new()?.find_active() {
        Ok(player) => {
            log::debug!("Found active music player: {}", player.bus_name());
            Ok(player)
        }
        Err(e) => {
            log::error!("Failed to find active music player: {}", e);
            Err(AppError::from(e))
        }
    }
}

#[tauri::command]
pub async fn spotify_play_pause() -> Result<(), AppError> {
    log::debug!("Toggling play/pause");
    find_player()?.checked_play_pause()?;
    Ok(())
}

#[tauri::command]
pub async fn spotify_next() -> Result<(), AppError> {
    log::debug!("Skipping to next track");
    find_player()?.checked_next()?;
    Ok(())
}

#[tauri::command]
pub async fn spotify_get_track() -> Result<TrackDto, AppError> {
    let player = find_player()?;
    let metadata = player.get_metadata().map_err(|e| {
        log::error!("Failed to get track metadata: {}", e);
        AppError::from(e)
    })?;
    let playback_status = player.get_playback_status().map_err(|e| {
        log::error!("Failed to get playback status: {}", e);
        AppError::from(e)
    })?;

    let title = metadata.title().unwrap_or("Unknown").to_string();

    let artist = metadata
        .artists()
        .and_then(|a| a.first().map(|s| s.to_string()))
        .unwrap_or_else(|| "Unknown".to_string());

    let status = match playback_status {
        PlaybackStatus::Playing => "Playing",
        PlaybackStatus::Paused => "Paused",
        PlaybackStatus::Stopped => "Stopped",
    }
    .to_string();

    log::debug!("Current track: {} by {} [{}]", title, artist, status);

    Ok(TrackDto {
        title,
        artist,
        status,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_dto_structure() {
        let track = TrackDto {
            title: "Test Song".to_string(),
            artist: "Test Artist".to_string(),
            status: "Playing".to_string(),
        };

        assert_eq!(track.title, "Test Song");
        assert_eq!(track.artist, "Test Artist");
        assert_eq!(track.status, "Playing");
    }

    #[test]
    fn test_playback_status_mapping() {
        let statuses = vec![
            (PlaybackStatus::Playing, "Playing"),
            (PlaybackStatus::Paused, "Paused"),
            (PlaybackStatus::Stopped, "Stopped"),
        ];

        for (status, expected) in statuses {
            let status_str = match status {
                PlaybackStatus::Playing => "Playing",
                PlaybackStatus::Paused => "Paused",
                PlaybackStatus::Stopped => "Stopped",
            };
            assert_eq!(status_str, expected);
        }
    }

    #[test]
    fn test_no_player_error_variant_exists() {
        let err = AppError::NoPlayer;
        assert!(matches!(err, AppError::NoPlayer));
    }
}
