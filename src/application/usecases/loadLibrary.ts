import type { Photo } from "../../domain/photo";
import type { PhotoCache } from "../ports";

export async function loadLibrary(
  cache: PhotoCache
): Promise<Photo[]> {
  const photos = await cache.getCachedPhotos();

  // Shuffle for variety each session
  for (let i = photos.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [photos[i], photos[j]] = [photos[j], photos[i]];
  }

  return photos;
}
