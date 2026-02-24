import type { Photo } from "../domain/photo";

export interface PhotoRepository {
  fetchPhotos(): Promise<Photo[]>;
}

export interface PhotoCache {
  getCachedPhotos(): Promise<Photo[]>;
  fetchAndCache(): Promise<Photo[]>;
}

export interface SlideshowClock {
  start(): void;
  stop(): void;
}

export interface MusicController {
  playPause(): Promise<void>;
  next(): Promise<void>;
  getCurrentTrack(): Promise<import("../domain/track").Track | null>;
}
