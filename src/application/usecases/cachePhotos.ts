import type { Photo } from "../../domain/photo";
import type { PhotoCache } from "../ports";

export async function cachePhotos(
  cache: PhotoCache
): Promise<Photo[]> {
  return cache.fetchAndCache();
}
