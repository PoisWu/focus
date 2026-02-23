import { describe, it, expect, beforeEach } from "vitest";
import { get } from "svelte/store";
import { slideshowStore } from "./slideshowStore";
import type { Photo } from "../domain/photo";

describe("slideshowStore", () => {
  beforeEach(() => {
    slideshowStore.reset();
  });

  it("should initialize with correct default state", () => {
    const state = get(slideshowStore);

    expect(state.current).toBeNull();
    expect(state.queue).toEqual([]);
    expect(state.paused).toBe(false);
    expect(state.secondsLeft).toBe(30);
  });

  it("should update current photo", () => {
    const photo: Photo = {
      id: "1",
      url: "https://images.unsplash.com/photo-1",
      photographer: "Alice",
      profileUrl: "https://unsplash.com/@alice",
    };

    slideshowStore.setPhoto(photo);
    const state = get(slideshowStore);

    expect(state.current).toEqual(photo);
  });

  it("should toggle pause state", () => {
    const initialState = get(slideshowStore);
    expect(initialState.paused).toBe(false);

    slideshowStore.togglePause();
    expect(get(slideshowStore).paused).toBe(true);

    slideshowStore.togglePause();
    expect(get(slideshowStore).paused).toBe(false);
  });

  it("should tick seconds down", () => {
    slideshowStore.resetTimer();
    expect(get(slideshowStore).secondsLeft).toBe(30);

    slideshowStore.tickSecond();
    expect(get(slideshowStore).secondsLeft).toBe(29);

    slideshowStore.tickSecond();
    expect(get(slideshowStore).secondsLeft).toBe(28);
  });

  it("should reset timer to 30 seconds", () => {
    slideshowStore.tickSecond();
    slideshowStore.tickSecond();
    expect(get(slideshowStore).secondsLeft).toBe(28);

    slideshowStore.resetTimer();
    expect(get(slideshowStore).secondsLeft).toBe(30);
  });
});
