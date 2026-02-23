import type { Photo } from "../../domain/photo";
import type { PhotoRepository } from "../ports";

export async function prefetchPhotos(
  repository: PhotoRepository,
  currentQueue: Photo[]
): Promise<Photo[]> {
  const newPhotos = await repository.fetchPhotos();
  return [...currentQueue, ...newPhotos];
}
