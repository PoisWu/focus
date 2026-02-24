use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Manager;

use crate::error::AppError;
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
        Ok(content) => match serde_json::from_str(&content) {
            Ok(entries) => entries,
            Err(e) => {
                log::warn!("Failed to parse photo metadata at {}: {}", path.display(), e);
                Vec::new()
            }
        },
        Err(e) => {
            log::debug!("No photo metadata file found at {}: {}", path.display(), e);
            Vec::new()
        }
    }
}

fn write_metadata(app: &AppHandle, entries: &[PhotoMetadata]) -> Result<(), std::io::Error> {
    let dir = cache_dir(app);
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(entries).map_err(std::io::Error::other)?;
    fs::write(metadata_path(app), json)
}

/// Returns up to `count` photos from the cache using a time-based offset for variety.
fn get_cached_sample(entries: &[PhotoMetadata], dir: &Path, count: usize) -> Vec<PhotoDto> {
    let valid: Vec<&PhotoMetadata> = entries
        .iter()
        .filter(|e| dir.join(&e.filename).exists())
        .collect();

    if valid.is_empty() {
        return Vec::new();
    }

    let offset = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as usize)
        .unwrap_or(0)
        % valid.len();

    let take = count.min(valid.len());
    (0..take)
        .map(|i| {
            let e = valid[(offset + i) % valid.len()];
            PhotoDto {
                id: e.id.clone(),
                url: dir.join(&e.filename).to_string_lossy().to_string(),
                photographer: e.photographer.clone(),
                profile_url: e.profile_url.clone(),
            }
        })
        .collect()
}

/// Fetches 10 photos from Unsplash, downloads them to disk, updates the metadata cache,
/// and returns only the newly cached photos.
async fn try_fetch_and_cache(
    app: &AppHandle,
    existing: &[PhotoMetadata],
    dir: &Path,
) -> Result<Vec<PhotoDto>, AppError> {
    let existing_ids: HashSet<String> = existing.iter().map(|e| e.id.clone()).collect();

    let access_key = std::env::var("UNSPLASH_ACCESS_KEY").map_err(|_| {
        log::error!("UNSPLASH_ACCESS_KEY environment variable is not set");
        AppError::Cache("UNSPLASH_ACCESS_KEY not set".into())
    })?;

    let url = format!(
        "https://api.unsplash.com/photos/random?count=10&query=nature&client_id={}",
        access_key
    );

    log::debug!("Fetching photos from Unsplash");
    let client = reqwest::Client::new();
    let api_photos: Vec<serde_json::Value> = client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            log::error!("Unsplash API request failed: {}", e);
            AppError::Unsplash(e.to_string())
        })?
        .json()
        .await
        .map_err(|e| {
            log::error!("Failed to deserialize Unsplash response: {}", e);
            AppError::Unsplash(e.to_string())
        })?;

    log::debug!("Received {} photos from Unsplash", api_photos.len());
    fs::create_dir_all(dir).map_err(|e| {
        log::error!("Failed to create cache directory {}: {}", dir.display(), e);
        AppError::Cache(e.to_string())
    })?;

    let mut new_metadata = existing.to_vec();
    let mut new_dtos: Vec<PhotoDto> = Vec::new();

    for p in api_photos {
        let id = p["id"].as_str().unwrap_or("").to_string();
        if id.is_empty() || existing_ids.contains(&id) {
            continue;
        }

        let download_url = p["urls"]["regular"].as_str().unwrap_or("").to_string();
        if download_url.is_empty() {
            log::warn!("Photo {} has no download URL, skipping", id);
            continue;
        }

        let image_bytes = match client.get(&download_url).send().await {
            Ok(resp) => match resp.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    log::warn!("Failed to read image bytes for photo {}: {}", id, e);
                    continue;
                }
            },
            Err(e) => {
                log::warn!("Failed to download photo {}: {}", id, e);
                continue;
            }
        };

        let filename = format!("{}.jpg", id);
        let file_path = dir.join(&filename);

        if let Err(e) = fs::write(&file_path, &image_bytes) {
            log::warn!("Failed to write photo {} to disk at {}: {}", id, file_path.display(), e);
            continue;
        }

        let metadata = PhotoMetadata {
            id: id.clone(),
            photographer: p["user"]["name"].as_str().unwrap_or("Unknown").to_string(),
            profile_url: p["user"]["links"]["html"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            filename,
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

    if let Err(e) = write_metadata(app, &new_metadata) {
        log::warn!("Failed to persist photo metadata: {}", e);
    }

    if new_dtos.is_empty() {
        log::warn!("No new photos were downloaded in this fetch cycle");
        return Err(AppError::Cache("no new photos downloaded".into()));
    }

    log::info!("Cached {} new photos", new_dtos.len());
    Ok(new_dtos)
}

#[tauri::command]
pub async fn get_cached_photos(app: AppHandle) -> Result<Vec<PhotoDto>, AppError> {
    let entries = read_metadata(&app);
    let dir = cache_dir(&app);
    log::debug!("Returning {} cached photos", entries.len());
    Ok(get_cached_sample(&entries, &dir, usize::MAX))
}

/// Fetches the next batch of photos, caching them to disk as they arrive.
///
/// Fallback scenarios:
/// - Scenario 3 (cache full): returns 10 cached photos without hitting the network.
/// - Scenario 2 (network error): returns up to 10 cached photos instead.
///
/// Returns an error only when the network is unreachable **and** the cache is empty.
#[tauri::command]
pub async fn fetch_photos(app: AppHandle) -> Result<Vec<PhotoDto>, AppError> {
    let existing = read_metadata(&app);
    let dir = cache_dir(&app);

    // Scenario 3: cache is full — serve from disk, skip the network
    if existing.len() >= MAX_PHOTOS {
        log::debug!("Cache is full ({} photos), serving from disk", existing.len());
        return Ok(get_cached_sample(&existing, &dir, 10));
    }

    // Normal path: fetch 10 fresh photos and accumulate in cache
    match try_fetch_and_cache(&app, &existing, &dir).await {
        Ok(photos) => Ok(photos),
        // Scenario 2: network failed — fall back to cached photos
        Err(e) => {
            log::warn!("Network fetch failed ({}), falling back to cache", e);
            let cached = get_cached_sample(&existing, &dir, 10);
            if cached.is_empty() {
                log::error!("No photos available: network unreachable and cache is empty");
                Err(AppError::Cache(
                    "No photos available: network unreachable and cache is empty".into(),
                ))
            } else {
                log::info!("Serving {} cached photos as fallback", cached.len());
                Ok(cached)
            }
        }
    }
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
        let result: Vec<PhotoMetadata> = serde_json::from_str("not valid json").unwrap_or_default();
        assert!(result.is_empty());
    }
}
