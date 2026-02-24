import { convertFileSrc } from "@tauri-apps/api/core";
import type { Photo } from "../domain/photo";

export interface PhotoDto {
  id: string;
  url: string;
  photographer: string;
  profile_url: string;
}

export function mapPhotoDto(dto: PhotoDto): Photo {
  return {
    id: dto.id,
    url: convertFileSrc(dto.url),
    photographer: dto.photographer,
    profileUrl: dto.profile_url,
  };
}
