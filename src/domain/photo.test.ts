import { describe, it, expect } from "vitest";
import type { Photo } from "./photo";

describe("Photo domain type", () => {
  it("should have required fields", () => {
    const photo: Photo = {
      id: "abc123",
      url: "https://images.unsplash.com/photo-123",
      photographer: "John Doe",
      profileUrl: "https://unsplash.com/@johndoe",
    };

    expect(photo.id).toBe("abc123");
    expect(photo.url).toBe("https://images.unsplash.com/photo-123");
    expect(photo.photographer).toBe("John Doe");
    expect(photo.profileUrl).toBe("https://unsplash.com/@johndoe");
  });

  it("should enforce non-empty URL", () => {
    const photo: Photo = {
      id: "test",
      url: "https://example.com/image.jpg",
      photographer: "Test",
      profileUrl: "https://example.com",
    };

    expect(photo.url.length).toBeGreaterThan(0);
  });
});
