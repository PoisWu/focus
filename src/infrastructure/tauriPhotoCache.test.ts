import { describe, it, expect, beforeEach, vi } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";
import { TauriPhotoCache } from "./tauriPhotoCache";

// Mock convertFileSrc since it's not available in test environment
beforeEach(() => {
  (window as any).__TAURI_INTERNALS__ = {
    ...(window as any).__TAURI_INTERNALS__,
    convertFileSrc: (path: string) => `http://asset.localhost/${path}`,
  };
});

describe("TauriPhotoCache", () => {
  beforeEach(() => {
    mockIPC((cmd) => {
      if (cmd === "get_cached_photos") {
        return Promise.resolve([
          {
            id: "cached1",
            url: "/home/user/.cache/com.cheng-yen.focus/photos/cached1.jpg",
            photographer: "Alice",
            profile_url: "https://unsplash.com/@alice",
          },
          {
            id: "cached2",
            url: "/home/user/.cache/com.cheng-yen.focus/photos/cached2.jpg",
            photographer: "Bob",
            profile_url: "https://unsplash.com/@bob",
          },
        ]);
      }
      if (cmd === "fetch_and_cache_photos") {
        return Promise.resolve([
          {
            id: "new1",
            url: "/home/user/.cache/com.cheng-yen.focus/photos/new1.jpg",
            photographer: "Charlie",
            profile_url: "https://unsplash.com/@charlie",
          },
        ]);
      }
    });
  });

  it("should load cached photos and map fields", async () => {
    const cache = new TauriPhotoCache();
    const photos = await cache.getCachedPhotos();

    expect(photos).toHaveLength(2);
    expect(photos[0].id).toBe("cached1");
    expect(photos[0].photographer).toBe("Alice");
    expect(photos[0]).toHaveProperty("profileUrl");
    expect(photos[0]).not.toHaveProperty("profile_url");
  });

  it("should fetch and cache new photos and map fields", async () => {
    const cache = new TauriPhotoCache();
    const photos = await cache.fetchAndCache();

    expect(photos).toHaveLength(1);
    expect(photos[0].id).toBe("new1");
    expect(photos[0].photographer).toBe("Charlie");
    expect(photos[0]).toHaveProperty("profileUrl");
  });
});
