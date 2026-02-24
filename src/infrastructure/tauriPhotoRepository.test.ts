import { describe, it, expect, beforeEach, vi } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";
import type { Photo } from "../domain/photo";
import { TauriPhotoRepository } from "./tauriPhotoRepository";

describe("TauriPhotoRepository", () => {
  beforeEach(() => {
    // convertFileSrc is not available in the test environment; pass URLs through unchanged
    (window as any).__TAURI_INTERNALS__ = {
      ...(window as any).__TAURI_INTERNALS__,
      convertFileSrc: (path: string) => path,
    };

    mockIPC((cmd, args) => {
      if (cmd === "fetch_photos") {
        return Promise.resolve([
          {
            id: "1",
            url: "https://images.unsplash.com/photo-1",
            photographer: "Alice",
            profile_url: "https://unsplash.com/@alice",
          },
          {
            id: "2",
            url: "https://images.unsplash.com/photo-2",
            photographer: "Bob",
            profile_url: "https://unsplash.com/@bob",
          },
        ]);
      }
    });
  });

  it("should fetch photos from Tauri backend", async () => {
    const repo = new TauriPhotoRepository();
    const photos = await repo.fetchPhotos();

    expect(photos).toHaveLength(2);
    expect(photos[0].id).toBe("1");
    expect(photos[0].url).toBe("https://images.unsplash.com/photo-1");
    expect(photos[0].photographer).toBe("Alice");
    expect(photos[0].profileUrl).toBe("https://unsplash.com/@alice");
  });

  it("should map profile_url to profileUrl", async () => {
    const repo = new TauriPhotoRepository();
    const photos = await repo.fetchPhotos();

    expect(photos[0]).toHaveProperty("profileUrl");
    expect(photos[0]).not.toHaveProperty("profile_url");
  });
});
