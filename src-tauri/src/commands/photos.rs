#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photo_dto_structure() {
        let photo = PhotoDto {
            id: "test123".to_string(),
            url: "https://images.unsplash.com/photo-123".to_string(),
            photographer: "Test User".to_string(),
            profile_url: "https://unsplash.com/@testuser".to_string(),
        };

        assert_eq!(photo.id, "test123");
        assert_eq!(photo.url, "https://images.unsplash.com/photo-123");
        assert_eq!(photo.photographer, "Test User");
        assert_eq!(photo.profile_url, "https://unsplash.com/@testuser");
    }

    #[test]
    fn test_env_var_missing_returns_error() {
        std::env::remove_var("UNSPLASH_ACCESS_KEY");
        
        let err = AppError::Unsplash("UNSPLASH_ACCESS_KEY not set".to_string());
        assert!(matches!(err, AppError::Unsplash(_)));
    }
}
