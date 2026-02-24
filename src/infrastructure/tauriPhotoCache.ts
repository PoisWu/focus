import { invoke } from "@tauri-apps/api/core";
import type { PhotoCache } from "../application/ports";
import type { Photo } from "../domain/photo";
import { type PhotoDto, mapPhotoDto } from "./photoAdapter";

export class TauriPhotoCache implements PhotoCache {
  async getCachedPhotos(): Promise<Photo[]> {
    const photos = await invoke<PhotoDto[]>("get_cached_photos");
    return photos.map(mapPhotoDto);
  }
}

