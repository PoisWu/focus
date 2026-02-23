import { describe, it, expect, vi } from "vitest";
import type { Photo } from "../../domain/photo";
import type { PhotoRepository } from "../ports";
import { prefetchPhotos } from "./prefetchPhotos";

describe("prefetchPhotos use case", () => {
  it("should fetch photos and add to queue", async () => {
    const mockPhotos: Photo[] = [
      {
        id: "1",
        url: "https://images.unsplash.com/photo-1",
        photographer: "Alice",
        profileUrl: "https://unsplash.com/@alice",
      },
      {
        id: "2",
        url: "https://images.unsplash.com/photo-2",
        photographer: "Bob",
        profileUrl: "https://unsplash.com/@bob",
      },
    ];

    const mockRepo: PhotoRepository = {
      fetchPhotos: vi.fn().mockResolvedValue(mockPhotos),
    };

    const currentQueue: Photo[] = [];
    const newQueue = await prefetchPhotos(mockRepo, currentQueue);

    expect(mockRepo.fetchPhotos).toHaveBeenCalledOnce();
    expect(newQueue).toHaveLength(2);
    expect(newQueue[0].id).toBe("1");
    expect(newQueue[1].id).toBe("2");
  });

  it("should append to existing queue", async () => {
    const existingPhoto: Photo = {
      id: "existing",
      url: "https://images.unsplash.com/photo-existing",
      photographer: "Charlie",
      profileUrl: "https://unsplash.com/@charlie",
    };

    const newPhoto: Photo = {
      id: "new",
      url: "https://images.unsplash.com/photo-new",
      photographer: "Dave",
      profileUrl: "https://unsplash.com/@dave",
    };

    const mockRepo: PhotoRepository = {
      fetchPhotos: vi.fn().mockResolvedValue([newPhoto]),
    };

    const currentQueue: Photo[] = [existingPhoto];
    const newQueue = await prefetchPhotos(mockRepo, currentQueue);

    expect(newQueue).toHaveLength(2);
    expect(newQueue[0].id).toBe("existing");
    expect(newQueue[1].id).toBe("new");
  });

  it("should handle empty response gracefully", async () => {
    const mockRepo: PhotoRepository = {
      fetchPhotos: vi.fn().mockResolvedValue([]),
    };

    const currentQueue: Photo[] = [];
    const newQueue = await prefetchPhotos(mockRepo, currentQueue);

    expect(newQueue).toHaveLength(0);
  });
});
