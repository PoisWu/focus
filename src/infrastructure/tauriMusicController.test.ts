import { describe, it, expect, beforeEach, vi } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";
import { TauriMusicController } from "./tauriMusicController";

describe("TauriMusicController", () => {
  beforeEach(() => {
    mockIPC((cmd, args) => {
      if (cmd === "spotify_get_track") {
        return Promise.resolve({
          title: "Test Song",
          artist: "Test Artist",
          status: "Playing",
        });
      }
      if (cmd === "spotify_play_pause") {
        return Promise.resolve();
      }
      if (cmd === "spotify_next") {
        return Promise.resolve();
      }
    });
  });

  it("should get current track from backend", async () => {
    const controller = new TauriMusicController();
    const track = await controller.getCurrentTrack();

    expect(track).not.toBeNull();
    expect(track?.title).toBe("Test Song");
    expect(track?.artist).toBe("Test Artist");
    expect(track?.status).toBe("Playing");
  });

  it("should return null when no player is found", async () => {
    mockIPC((cmd) => {
      if (cmd === "spotify_get_track") {
        return Promise.reject(new Error("No music player found"));
      }
    });

    const controller = new TauriMusicController();
    const track = await controller.getCurrentTrack();

    expect(track).toBeNull();
  });

  it("should call play pause command", async () => {
    const controller = new TauriMusicController();
    await expect(controller.playPause()).resolves.not.toThrow();
  });

  it("should call next command", async () => {
    const controller = new TauriMusicController();
    await expect(controller.next()).resolves.not.toThrow();
  });
});
