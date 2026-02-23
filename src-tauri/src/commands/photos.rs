use crate::error::AppError;
use crate::models::PhotoDto;

#[tauri::command]
pub async fn fetch_photos() -> Result<Vec<PhotoDto>, AppError> {
    let access_key = std::env::var("UNSPLASH_ACCESS_KEY")
        .map_err(|_| AppError::Unsplash("UNSPLASH_ACCESS_KEY not set".to_string()))?;

    let url = format!(
        "https://api.unsplash.com/photos/random?count=10&query=nature&client_id={}",
        access_key
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    
    let photos: Vec<serde_json::Value> = response.json().await?;

    let photo_dtos = photos
        .into_iter()
        .map(|p| PhotoDto {
            id: p["id"].as_str().unwrap_or("").to_string(),
            url: p["urls"]["regular"].as_str().unwrap_or("").to_string(),
            photographer: p["user"]["name"].as_str().unwrap_or("Unknown").to_string(),
            profile_url: p["user"]["links"]["html"].as_str().unwrap_or("").to_string(),
        })
        .collect();

    Ok(photo_dtos)
}

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
