<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { get } from "svelte/store";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { error as logError, warn as logWarn, debug as logDebug, info as logInfo } from "@tauri-apps/plugin-log";
  import PhotoFrame from "../presentation/components/PhotoFrame.svelte";
  import TimerDot from "../presentation/components/TimerDot.svelte";
  import MusicHud from "../presentation/components/MusicHud.svelte";
  import { slideshowStore } from "../infrastructure/slideshowStore";
  import { musicStore } from "../infrastructure/musicStore";
  import { TauriPhotoRepository } from "../infrastructure/tauriPhotoRepository";
  import { TauriPhotoCache } from "../infrastructure/tauriPhotoCache";
  import { TauriMusicController } from "../infrastructure/tauriMusicController";
  import { advanceSlide } from "../application/usecases/advanceSlide";
  import { loadLibrary } from "../application/usecases/loadLibrary";


  const photoRepo = new TauriPhotoRepository();
  const photoCache = new TauriPhotoCache();
  const musicController = new TauriMusicController();

  let intervalId: number | null = null;
  let musicPollId: number | null = null;
  let hudTimeoutId: number | null = null;
  let error: string | null = null;

  onMount(async () => {
    // Force fullscreen (static config not always respected on Linux)
    const win = getCurrentWindow();
    await win.maximize();
    await win.setFullscreen(true);

    // Load initial batch: use cache for instant start, or fetch from network on first launch
    try {
      const cachedPhotos = await loadLibrary(photoCache);
      await logDebug(`Loaded ${cachedPhotos.length} photos from cache`);

      const startup = cachedPhotos.length > 0
        ? cachedPhotos
        : await photoRepo.fetchPhotos();

      if (cachedPhotos.length === 0) {
        await logInfo(`Cache empty — fetched ${startup.length} photos from network`);
      }

      const result = await advanceSlide(photoRepo, startup);
      await logInfo(`Startup: showing photo "${result.current.id}", queue has ${result.queue.length} photo(s)`);
      slideshowStore.setPhoto(result.current);
      slideshowStore.setQueue(result.queue);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      await logError(`Failed to load photos: ${e}`);
      return;
    }

    // Start timer
    intervalId = window.setInterval(async () => {
      const state = get(slideshowStore);

      if (!state || state.paused) return;

      if (state.secondsLeft <= 1) {
        await advance();
      } else {
        slideshowStore.tickSecond();
      }
    }, 1000);

  });

  onDestroy(() => {
    if (intervalId) clearInterval(intervalId);
    if (musicPollId) clearInterval(musicPollId);
    if (hudTimeoutId) clearTimeout(hudTimeoutId);
  });

  async function advance() {
    const state = get(slideshowStore);

    if (!state) return;

    await logDebug(`Advancing slide, queue has ${state.queue.length} photo(s) remaining`);
    const result = await advanceSlide(photoRepo, state.queue);
    await logDebug(`Now showing photo "${result.current.id}", queue refilled to ${result.queue.length} photo(s)`);

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

      const hudState = get(musicStore);

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
