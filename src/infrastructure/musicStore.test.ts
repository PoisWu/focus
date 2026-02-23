import { describe, it, expect, beforeEach } from "vitest";
import { get } from "svelte/store";
import { musicStore } from "./musicStore";
import type { Track } from "../domain/track";

describe("musicStore", () => {
  beforeEach(() => {
    musicStore.reset();
  });

  it("should initialize with default state", () => {
    const state = get(musicStore);

    expect(state.track).toBeNull();
    expect(state.hudVisible).toBe(false);
  });

  it("should set track", () => {
    const track: Track = {
      title: "Test Song",
      artist: "Test Artist",
      status: "Playing",
    };

    musicStore.setTrack(track);
    const state = get(musicStore);

    expect(state.track).toEqual(track);
  });

  it("should toggle HUD visibility", () => {
    const initialState = get(musicStore);
    expect(initialState.hudVisible).toBe(false);

    musicStore.toggleHud();
    expect(get(musicStore).hudVisible).toBe(true);

    musicStore.toggleHud();
    expect(get(musicStore).hudVisible).toBe(false);
  });

  it("should clear track", () => {
    const track: Track = {
      title: "Test Song",
      artist: "Test Artist",
      status: "Playing",
    };

    musicStore.setTrack(track);
    expect(get(musicStore).track).toEqual(track);

    musicStore.setTrack(null);
    expect(get(musicStore).track).toBeNull();
  });
});
