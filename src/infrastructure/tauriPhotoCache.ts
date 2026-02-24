import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import type { PhotoCache } from "../application/ports";
import type { Photo } from "../domain/photo";

interface PhotoDto {
  id: string;
  url: string;
  photographer: string;
  profile_url: string;
}

export class TauriPhotoCache implements PhotoCache {
  async getCachedPhotos(): Promise<Photo[]> {
    const photos = await invoke<PhotoDto[]>("get_cached_photos");

    return photos.map((dto) => ({
      id: dto.id,
      url: convertFileSrc(dto.url),
      photographer: dto.photographer,
      profileUrl: dto.profile_url,
    }));
  }

  async fetchAndCache(): Promise<Photo[]> {
    const photos = await invoke<PhotoDto[]>("fetch_and_cache_photos");

    return photos.map((dto) => ({
      id: dto.id,
      url: convertFileSrc(dto.url),
      photographer: dto.photographer,
      profileUrl: dto.profile_url,
    }));
  }
}
