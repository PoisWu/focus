import { describe, it, expect, vi } from "vitest";
import type { Photo } from "../../domain/photo";
import type { PhotoCache } from "../ports";
import { cachePhotos } from "./cachePhotos";

describe("cachePhotos use case", () => {
  it("should delegate to cache.fetchAndCache", async () => {
    const newPhotos: Photo[] = [
      {
        id: "new1",
        url: "http://asset.localhost/new1.jpg",
        photographer: "Charlie",
        profileUrl: "https://unsplash.com/@charlie",
      },
    ];

    const mockCache: PhotoCache = {
      getCachedPhotos: vi.fn(),
      fetchAndCache: vi.fn().mockResolvedValue(newPhotos),
    };

    const result = await cachePhotos(mockCache);

    expect(mockCache.fetchAndCache).toHaveBeenCalledOnce();
    expect(result).toHaveLength(1);
    expect(result[0].id).toBe("new1");
  });

  it("should return empty array when library is full", async () => {
    const mockCache: PhotoCache = {
      getCachedPhotos: vi.fn(),
      fetchAndCache: vi.fn().mockResolvedValue([]),
    };

    const result = await cachePhotos(mockCache);

    expect(result).toHaveLength(0);
  });
});
