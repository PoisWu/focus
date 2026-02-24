<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import PhotoFrame from "../presentation/components/PhotoFrame.svelte";
  import TimerDot from "../presentation/components/TimerDot.svelte";
  import MusicHud from "../presentation/components/MusicHud.svelte";
  import { slideshowStore } from "../infrastructure/slideshowStore";
  import { musicStore } from "../infrastructure/musicStore";
  import { TauriPhotoRepository } from "../infrastructure/tauriPhotoRepository";
  import { TauriPhotoCache } from "../infrastructure/tauriPhotoCache";
  import { TauriMusicController } from "../infrastructure/tauriMusicController";
  import { advanceSlide } from "../application/usecases/advanceSlide";
  import { prefetchPhotos } from "../application/usecases/prefetchPhotos";
  import { loadLibrary } from "../application/usecases/loadLibrary";
  import { cachePhotos } from "../application/usecases/cachePhotos";
  import type { PhotoRepository } from "../application/ports";

  const photoRepo = new TauriPhotoRepository();
  const photoCache = new TauriPhotoCache();
  const musicController = new TauriMusicController();

  // Adapter: mid-session top-ups go through the cache layer
  const cacheAsRepo: PhotoRepository = {
    fetchPhotos: () => photoCache.fetchAndCache(),
  };

  let intervalId: number | null = null;
  let musicPollId: number | null = null;
  let hudTimeoutId: number | null = null;
  let error: string | null = null;

  onMount(async () => {
    // Force fullscreen (static config not always respected on Linux)
    const win = getCurrentWindow();
    await win.maximize();
    await win.setFullscreen(true);

    // Try loading from local cache first
    try {
      const cachedPhotos = await loadLibrary(photoCache);

      if (cachedPhotos.length > 0) {
        // Instant start from cache
        const result = await advanceSlide(cacheAsRepo, cachedPhotos);
        slideshowStore.setPhoto(result.current);
        slideshowStore.setQueue(result.queue);

        // Background: fetch more photos and add them to the queue
        cachePhotos(photoCache).then((newPhotos) => {
          if (newPhotos.length > 0) {
            const unsubscribe = slideshowStore.subscribe((s) => {
              if (s) {
                slideshowStore.setQueue([...s.queue, ...newPhotos]);
              }
            });
            unsubscribe();
          }
        }).catch(() => {});
      } else {
        // First launch: no cache, fetch from network
        const initialPhotos = await prefetchPhotos(photoRepo, []);
        const result = await advanceSlide(photoRepo, initialPhotos);
        slideshowStore.setPhoto(result.current);
        slideshowStore.setQueue(result.queue);

        // Background: cache the photos we just showed + more
        cachePhotos(photoCache).catch(() => {});
      }
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      console.error("Failed to load photos:", e);
      return;
    }

    // Start timer
    intervalId = window.setInterval(async () => {
      let state;
      const unsubscribe = slideshowStore.subscribe(s => state = s);
      unsubscribe();
      
      if (!state || state.paused) return;

      if (state.secondsLeft <= 1) {
        await advance();
      } else {
        slideshowStore.tickSecond();
      }
    }, 1000);

    return () => {
      if (intervalId) clearInterval(intervalId);
      if (musicPollId) clearInterval(musicPollId);
      if (hudTimeoutId) clearTimeout(hudTimeoutId);
    };
  });

  async function advance() {
    let state;
    const unsubscribe = slideshowStore.subscribe(s => state = s);
    unsubscribe();
    
    if (!state) return;
    
    const result = await advanceSlide(cacheAsRepo, state.queue);
    
    slideshowStore.setPhoto(result.current);
    slideshowStore.setQueue(result.queue);
    slideshowStore.resetTimer();
  }

  function handleKeyboard(event: KeyboardEvent) {
    if (event.key === "Escape") {
      getCurrentWindow().close();
    } else if (event.key === "ArrowRight") {
      advance();
    } else if (event.key === " ") {
      slideshowStore.togglePause();
    } else if (event.key === "m" || event.key === "M") {
      musicStore.toggleHud();
      
      let hudState;
      const unsubscribe = musicStore.subscribe(s => hudState = s);
      unsubscribe();
      
      if (!hudState) return;
      
      if (hudState.hudVisible) {
        updateTrack();
        if (!musicPollId) {
          musicPollId = window.setInterval(updateTrack, 5000);
        }
        
        if (hudTimeoutId) clearTimeout(hudTimeoutId);
        hudTimeoutId = window.setTimeout(() => {
          musicStore.hideHud();
        }, 3000);
      } else {
        if (musicPollId) {
          clearInterval(musicPollId);
          musicPollId = null;
        }
      }
    }
  }

  async function updateTrack() {
    const track = await musicController.getCurrentTrack();
    musicStore.setTrack(track);
  }

  async function handlePlayPause() {
    await musicController.playPause();
    setTimeout(updateTrack, 100);
  }

  async function handleNext() {
    await musicController.next();
    setTimeout(updateTrack, 100);
  }
</script>

<svelte:window onkeydown={handleKeyboard} />

<div class="app">
  {#if error}
    <div class="error">{error}</div>
  {:else if $slideshowStore.current}
    <PhotoFrame photo={$slideshowStore.current} />
    <TimerDot secondsLeft={$slideshowStore.secondsLeft} />
  {/if}

  <MusicHud
    track={$musicStore.track}
    visible={$musicStore.hudVisible}
    onPlayPause={handlePlayPause}
    onNext={handleNext}
  />
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    background: #000;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }

  .app {
    width: 100vw;
    height: 100vh;
    position: relative;
  }

  .error {
    color: #ff6b6b;
    font-size: 1rem;
    padding: 2rem;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    background: rgba(255, 0, 0, 0.1);
    border: 1px solid rgba(255, 107, 107, 0.4);
    border-radius: 8px;
  }
</style>
