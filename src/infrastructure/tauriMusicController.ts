import { invoke } from "@tauri-apps/api/core";
import { warn } from "@tauri-apps/plugin-log";
import type { MusicController } from "../application/ports";
import type { Track } from "../domain/track";

export class TauriMusicController implements MusicController {
  async playPause(): Promise<void> {
    await invoke("spotify_play_pause");
  }

  async next(): Promise<void> {
    await invoke("spotify_next");
  }

  async getCurrentTrack(): Promise<Track | null> {
    try {
      const track = await invoke<Track>("spotify_get_track");
      return track;
    } catch (error) {
      await warn(`Failed to get current track: ${error}`);
      return null;
    }
  }
}
