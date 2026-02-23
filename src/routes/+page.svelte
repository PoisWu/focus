<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import PhotoFrame from "../presentation/components/PhotoFrame.svelte";
  import TimerDot from "../presentation/components/TimerDot.svelte";
  import Attribution from "../presentation/components/Attribution.svelte";
  import MusicHud from "../presentation/components/MusicHud.svelte";
  import { slideshowStore } from "../infrastructure/slideshowStore";
  import { musicStore } from "../infrastructure/musicStore";
  import { TauriPhotoRepository } from "../infrastructure/tauriPhotoRepository";
  import { TauriMusicController } from "../infrastructure/tauriMusicController";
  import { advanceSlide } from "../application/usecases/advanceSlide";
  import { prefetchPhotos } from "../application/usecases/prefetchPhotos";

  const photoRepo = new TauriPhotoRepository();
  const musicController = new TauriMusicController();

  let intervalId: number | null = null;
  let musicPollId: number | null = null;
  let hudTimeoutId: number | null = null;

  onMount(async () => {
    // Initial photo load
    const initialPhotos = await prefetchPhotos(photoRepo, []);
    const result = await advanceSlide(photoRepo, initialPhotos);
    
    slideshowStore.setPhoto(result.current);
    slideshowStore.setQueue(result.queue);

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
    
    const result = await advanceSlide(photoRepo, state.queue);
    
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
  {#if $slideshowStore.current}
    <PhotoFrame photo={$slideshowStore.current} />
    <Attribution photo={$slideshowStore.current} />
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
</style>
