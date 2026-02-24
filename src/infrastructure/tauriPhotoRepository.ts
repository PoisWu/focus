import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import type { PhotoRepository } from "../application/ports";
import type { Photo } from "../domain/photo";

interface PhotoDto {
  id: string;
  url: string;
  photographer: string;
  profile_url: string;
}

export class TauriPhotoRepository implements PhotoRepository {
  async fetchPhotos(): Promise<Photo[]> {
    const photos = await invoke<PhotoDto[]>("fetch_photos");
    
    return photos.map((dto) => ({
      id: dto.id,
      url: convertFileSrc(dto.url),
      photographer: dto.photographer,
      profileUrl: dto.profile_url,
    }));
  }
}
