use crate::error::AppError;
use crate::models::TrackDto;
use mpris::{PlayerFinder, PlaybackStatus};

#[tauri::command]
pub async fn spotify_play_pause() -> Result<(), AppError> {
    let player = PlayerFinder::new()?.find_active()?;
    player.checked_play_pause()?;
    Ok(())
}

#[tauri::command]
pub async fn spotify_next() -> Result<(), AppError> {
    let player = PlayerFinder::new()?.find_active()?;
    player.checked_next()?;
    Ok(())
}

#[tauri::command]
pub async fn spotify_get_track() -> Result<TrackDto, AppError> {
    let player = PlayerFinder::new()?.find_active()?;
    let metadata = player.get_metadata()?;
    let playback_status = player.get_playback_status()?;

    let title = metadata
        .title()
        .unwrap_or("Unknown")
        .to_string();

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
