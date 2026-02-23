import { describe, it, expect } from "vitest";
import type { Track } from "./track";

describe("Track domain type", () => {
  it("should have valid status values", () => {
    const playingTrack: Track = {
      title: "Song Title",
      artist: "Artist Name",
      status: "Playing",
    };

    const pausedTrack: Track = {
      title: "Song Title",
      artist: "Artist Name",
      status: "Paused",
    };

    const stoppedTrack: Track = {
      title: "Song Title",
      artist: "Artist Name",
      status: "Stopped",
    };

    expect(playingTrack.status).toBe("Playing");
    expect(pausedTrack.status).toBe("Paused");
    expect(stoppedTrack.status).toBe("Stopped");
  });

  it("should have title and artist", () => {
    const track: Track = {
      title: "Amazing Song",
      artist: "Cool Band",
      status: "Playing",
    };

    expect(track.title).toBe("Amazing Song");
    expect(track.artist).toBe("Cool Band");
  });
});
