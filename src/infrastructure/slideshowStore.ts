import { writable } from "svelte/store";
import type { Photo } from "../domain/photo";

interface SlideshowState {
  current: Photo | null;
  queue: Photo[];
  paused: boolean;
  secondsLeft: number;
}

function createSlideshowStore() {
  const { subscribe, set, update } = writable<SlideshowState>({
    current: null,
    queue: [],
    paused: false,
    secondsLeft: 30,
  });

  return {
    subscribe,
    setPhoto: (photo: Photo) =>
      update((state) => ({ ...state, current: photo })),
    setQueue: (queue: Photo[]) => update((state) => ({ ...state, queue })),
    togglePause: () => update((state) => ({ ...state, paused: !state.paused })),
    tickSecond: () =>
      update((state) => ({ ...state, secondsLeft: state.secondsLeft - 1 })),
    resetTimer: () => update((state) => ({ ...state, secondsLeft: 30 })),
    reset: () =>
      set({ current: null, queue: [], paused: false, secondsLeft: 30 }),
  };
}

export const slideshowStore = createSlideshowStore();
