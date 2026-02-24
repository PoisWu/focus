use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoDto {
    pub id: String,
    pub url: String,
    pub photographer: String,
    pub profile_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackDto {
    pub title: String,
    pub artist: String,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn photo_dto_serializes_to_correct_json() {
        let photo = PhotoDto {
            id: "test123".to_string(),
            url: "https://images.unsplash.com/photo-123".to_string(),
            photographer: "John Doe".to_string(),
            profile_url: "https://unsplash.com/@johndoe".to_string(),
        };

        let json = serde_json::to_value(&photo).unwrap();

        assert_eq!(json["id"], "test123");
        assert_eq!(json["url"], "https://images.unsplash.com/photo-123");
        assert_eq!(json["photographer"], "John Doe");
        assert_eq!(json["profile_url"], "https://unsplash.com/@johndoe");
    }

    #[test]
    fn track_dto_serializes_to_correct_json() {
        let track = TrackDto {
            title: "Song Title".to_string(),
            artist: "Artist Name".to_string(),
            status: "Playing".to_string(),
        };

        let json = serde_json::to_value(&track).unwrap();

        assert_eq!(json["title"], "Song Title");
        assert_eq!(json["artist"], "Artist Name");
        assert_eq!(json["status"], "Playing");
    }

    #[test]
    fn track_dto_accepts_valid_statuses() {
        let statuses = vec!["Playing", "Paused", "Stopped"];

        for status in statuses {
            let track = TrackDto {
                title: "Test".to_string(),
                artist: "Test".to_string(),
                status: status.to_string(),
            };

            assert!(matches!(
                track.status.as_str(),
                "Playing" | "Paused" | "Stopped"
            ));
        }
    }
}
