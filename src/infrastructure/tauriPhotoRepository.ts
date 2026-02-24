import { invoke } from "@tauri-apps/api/core";
import type { PhotoRepository } from "../application/ports";
import type { Photo } from "../domain/photo";
import { type PhotoDto, mapPhotoDto } from "./photoAdapter";

export class TauriPhotoRepository implements PhotoRepository {
  async fetchPhotos(): Promise<Photo[]> {
    const photos = await invoke<PhotoDto[]>("fetch_photos");
    return photos.map(mapPhotoDto);
  }
}
