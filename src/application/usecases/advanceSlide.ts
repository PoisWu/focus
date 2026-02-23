import type { Photo } from "../../domain/photo";
import type { PhotoRepository } from "../ports";
import { prefetchPhotos } from "./prefetchPhotos";

export async function advanceSlide(
  repository: PhotoRepository,
  queue: Photo[]
): Promise<{ current: Photo; queue: Photo[] }> {
  const [current, ...remaining] = queue;

  let newQueue = remaining;

  if (remaining.length < 3) {
    newQueue = await prefetchPhotos(repository, remaining);
  }

  return {
    current,
    queue: newQueue,
  };
}
