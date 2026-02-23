import { describe, it, expect, vi } from "vitest";
import type { Photo } from "../../domain/photo";
import type { PhotoRepository } from "../ports";
import { advanceSlide } from "./advanceSlide";

describe("advanceSlide use case", () => {
  it("should return current photo and shrink queue by 1", async () => {
    const photos: Photo[] = [
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
      {
        id: "3",
        url: "https://images.unsplash.com/photo-3",
        photographer: "Charlie",
        profileUrl: "https://unsplash.com/@charlie",
      },
      {
        id: "4",
        url: "https://images.unsplash.com/photo-4",
        photographer: "Dave",
        profileUrl: "https://unsplash.com/@dave",
      },
    ];

    const mockRepo: PhotoRepository = {
      fetchPhotos: vi.fn().mockResolvedValue([]),
    };

    const result = await advanceSlide(mockRepo, photos);

    expect(result.current.id).toBe("1");
    expect(result.queue).toHaveLength(3);
    expect(result.queue[0].id).toBe("2");
    expect(result.queue[1].id).toBe("3");
    expect(result.queue[2].id).toBe("4");
  });

  it("should trigger prefetch when queue drops below 3", async () => {
    const photos: Photo[] = [
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

    const newPhoto: Photo = {
      id: "3",
      url: "https://images.unsplash.com/photo-3",
      photographer: "Charlie",
      profileUrl: "https://unsplash.com/@charlie",
    };

    const mockRepo: PhotoRepository = {
      fetchPhotos: vi.fn().mockResolvedValue([newPhoto]),
    };

    const result = await advanceSlide(mockRepo, photos);

    expect(result.current.id).toBe("1");
    expect(mockRepo.fetchPhotos).toHaveBeenCalled();
    expect(result.queue).toHaveLength(2);
    expect(result.queue[1].id).toBe("3");
  });

  it("should return correct current and queue structure", async () => {
    const photos: Photo[] = [
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
      {
        id: "3",
        url: "https://images.unsplash.com/photo-3",
        photographer: "Charlie",
        profileUrl: "https://unsplash.com/@charlie",
      },
      {
        id: "4",
        url: "https://images.unsplash.com/photo-4",
        photographer: "Dave",
        profileUrl: "https://unsplash.com/@dave",
      },
    ];

    const mockRepo: PhotoRepository = {
      fetchPhotos: vi.fn().mockResolvedValue([]),
    };

    const result = await advanceSlide(mockRepo, photos);

    expect(result).toHaveProperty("current");
    expect(result).toHaveProperty("queue");
    expect(result.current.id).toBe("1");
    expect(Array.isArray(result.queue)).toBe(true);
  });
});
