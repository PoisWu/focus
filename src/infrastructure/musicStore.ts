import { writable } from "svelte/store";
import type { Track } from "../domain/track";

interface MusicState {
  track: Track | null;
  hudVisible: boolean;
}

function createMusicStore() {
  const { subscribe, update, set } = writable<MusicState>({
    track: null,
    hudVisible: false,
  });

  return {
    subscribe,
    setTrack: (track: Track | null) =>
      update((state) => ({ ...state, track })),
    toggleHud: () =>
      update((state) => ({ ...state, hudVisible: !state.hudVisible })),
    showHud: () => update((state) => ({ ...state, hudVisible: true })),
    hideHud: () => update((state) => ({ ...state, hudVisible: false })),
    reset: () => set({ track: null, hudVisible: false }),
  };
}

export const musicStore = createMusicStore();
