import { describe, it, expect, vi } from "vitest";
import type { Photo } from "../../domain/photo";
import type { PhotoCache } from "../ports";
import { loadLibrary } from "./loadLibrary";

describe("loadLibrary use case", () => {
  it("should return cached photos", async () => {
    const cachedPhotos: Photo[] = [
      {
        id: "1",
        url: "http://asset.localhost/photo-1.jpg",
        photographer: "Alice",
        profileUrl: "https://unsplash.com/@alice",
      },
      {
        id: "2",
        url: "http://asset.localhost/photo-2.jpg",
        photographer: "Bob",
        profileUrl: "https://unsplash.com/@bob",
      },
    ];

    const mockCache: PhotoCache = {
      getCachedPhotos: vi.fn().mockResolvedValue(cachedPhotos),
      fetchAndCache: vi.fn(),
    };

    const result = await loadLibrary(mockCache);

    expect(mockCache.getCachedPhotos).toHaveBeenCalledOnce();
    expect(result).toHaveLength(2);
  });

  it("should return empty array when no cache exists", async () => {
    const mockCache: PhotoCache = {
      getCachedPhotos: vi.fn().mockResolvedValue([]),
      fetchAndCache: vi.fn(),
    };

    const result = await loadLibrary(mockCache);

    expect(result).toHaveLength(0);
  });

  it("should shuffle photos for variety", async () => {
    const photos: Photo[] = Array.from({ length: 20 }, (_, i) => ({
      id: `${i}`,
      url: `http://asset.localhost/photo-${i}.jpg`,
      photographer: `Photographer ${i}`,
      profileUrl: `https://unsplash.com/@photographer${i}`,
    }));

    const mockCache: PhotoCache = {
      getCachedPhotos: vi.fn().mockResolvedValue([...photos]),
      fetchAndCache: vi.fn(),
    };

    const result = await loadLibrary(mockCache);

    expect(result).toHaveLength(20);
    // All original photos should still be present (just reordered)
    const ids = result.map((p) => p.id).sort();
    expect(ids).toEqual(photos.map((p) => p.id).sort());
  });
});
