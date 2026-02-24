use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Manager;

use crate::models::PhotoDto;

const MAX_PHOTOS: usize = 1000;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PhotoMetadata {
    id: String,
    photographer: String,
    profile_url: String,
    filename: String,
}

fn cache_dir(app: &AppHandle) -> PathBuf {
    let base = app
        .path()
        .app_cache_dir()
        .expect("failed to resolve app cache dir");
    base.join("photos")
}

fn metadata_path(app: &AppHandle) -> PathBuf {
    cache_dir(app).join("metadata.json")
}

fn read_metadata(app: &AppHandle) -> Vec<PhotoMetadata> {
    let path = metadata_path(app);
    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

fn write_metadata(app: &AppHandle, entries: &[PhotoMetadata]) -> Result<(), std::io::Error> {
    let dir = cache_dir(app);
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(entries)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(metadata_path(app), json)
}

#[tauri::command]
pub async fn get_cached_photos(app: AppHandle) -> Result<Vec<PhotoDto>, String> {
    let entries = read_metadata(&app);
    let dir = cache_dir(&app);

    let photos = entries
        .into_iter()
        .filter(|entry| dir.join(&entry.filename).exists())
        .map(|entry| PhotoDto {
            id: entry.id,
            url: dir.join(&entry.filename).to_string_lossy().to_string(),
            photographer: entry.photographer,
            profile_url: entry.profile_url,
        })
        .collect();

    Ok(photos)
}

#[tauri::command]
pub async fn fetch_and_cache_photos(app: AppHandle) -> Result<Vec<PhotoDto>, String> {
    let existing = read_metadata(&app);

    if existing.len() >= MAX_PHOTOS {
        return Ok(Vec::new());
    }

    let existing_ids: HashSet<String> = existing.iter().map(|e| e.id.clone()).collect();

    // Fetch from Unsplash
    let access_key = match std::env::var("UNSPLASH_ACCESS_KEY") {
        Ok(key) => key,
        Err(_) => return Ok(Vec::new()),
    };

    let url = format!(
        "https://api.unsplash.com/photos/random?count=10&query=nature&client_id={}",
        access_key
    );

    let client = reqwest::Client::new();

    let response = match client.get(&url).send().await {
        Ok(r) => r,
        Err(_) => return Ok(Vec::new()),
    };

    let api_photos: Vec<serde_json::Value> = match response.json().await {
        Ok(p) => p,
        Err(_) => return Ok(Vec::new()),
    };

    let dir = cache_dir(&app);
    if let Err(_) = fs::create_dir_all(&dir) {
        return Ok(Vec::new());
    }

    let mut new_metadata = existing.clone();
    let mut new_dtos: Vec<PhotoDto> = Vec::new();

    for p in api_photos {
        let id = p["id"].as_str().unwrap_or("").to_string();
        if id.is_empty() || existing_ids.contains(&id) {
            continue;
        }

        let download_url = p["urls"]["regular"].as_str().unwrap_or("").to_string();
        if download_url.is_empty() {
            continue;
        }

        // Download image bytes
        let image_bytes = match client.get(&download_url).send().await {
            Ok(resp) => match resp.bytes().await {
                Ok(b) => b,
                Err(_) => continue,
            },
            Err(_) => continue,
        };

        let filename = format!("{}.jpg", id);
        let file_path = dir.join(&filename);

        if let Err(_) = fs::write(&file_path, &image_bytes) {
            continue;
        }

        let metadata = PhotoMetadata {
            id: id.clone(),
            photographer: p["user"]["name"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),
            profile_url: p["user"]["links"]["html"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            filename: filename.clone(),
        };

        new_dtos.push(PhotoDto {
            id,
            url: file_path.to_string_lossy().to_string(),
            photographer: metadata.photographer.clone(),
            profile_url: metadata.profile_url.clone(),
        });

        new_metadata.push(metadata);

        if new_metadata.len() >= MAX_PHOTOS {
            break;
        }
    }

    // Persist updated metadata
    let _ = write_metadata(&app, &new_metadata);

    Ok(new_dtos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photo_metadata_serialization() {
        let metadata = PhotoMetadata {
            id: "abc123".to_string(),
            photographer: "Alice".to_string(),
            profile_url: "https://unsplash.com/@alice".to_string(),
            filename: "abc123.jpg".to_string(),
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: PhotoMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, "abc123");
        assert_eq!(deserialized.photographer, "Alice");
        assert_eq!(deserialized.filename, "abc123.jpg");
    }

    #[test]
    fn test_max_photos_constant() {
        assert_eq!(MAX_PHOTOS, 1000);
    }

    #[test]
    fn test_metadata_deserialization_from_json() {
        let json = r#"[
            {
                "id": "photo1",
                "photographer": "Bob",
                "profile_url": "https://unsplash.com/@bob",
                "filename": "photo1.jpg"
            }
        ]"#;

        let entries: Vec<PhotoMetadata> = serde_json::from_str(json).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, "photo1");
    }

    #[test]
    fn test_invalid_json_returns_empty() {
        let result: Vec<PhotoMetadata> =
            serde_json::from_str("not valid json").unwrap_or_default();
        assert!(result.is_empty());
    }
}
