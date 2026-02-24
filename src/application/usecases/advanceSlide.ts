import type { Photo } from "../../domain/photo";
import type { PhotoRepository } from "../ports";

export async function advanceSlide(
  repository: PhotoRepository,
  queue: Photo[]
): Promise<{ current: Photo; queue: Photo[] }> {
  const [current, ...remaining] = queue;

  let newQueue = remaining;

  if (remaining.length < 3) {
    const fresh = await repository.fetchPhotos();
    newQueue = [...remaining, ...fresh];
  }

  return {
    current,
    queue: newQueue,
  };
}
