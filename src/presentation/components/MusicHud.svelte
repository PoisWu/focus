<script lang="ts">
  import { fade } from "svelte/transition";
  import type { Track } from "../../domain/track";

  let {
    track,
    visible,
    onPlayPause,
    onNext,
  }: {
    track: Track | null;
    visible: boolean;
    onPlayPause: () => void;
    onNext: () => void;
  } = $props();
</script>

{#if visible && track}
  <div class="music-hud" transition:fade={{ duration: 300 }}>
    <div class="track-info">
      <div class="title">{track.title}</div>
      <div class="artist">{track.artist}</div>
    </div>
    <div class="controls">
      <button onclick={onPlayPause}>
        {track.status === "Playing" ? "⏸" : "▶"}
      </button>
      <button onclick={onNext}>⏭</button>
    </div>
  </div>
{/if}

<style>
  .music-hud {
    position: fixed;
    top: 2rem;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(10px);
    padding: 1rem 1.5rem;
    border-radius: 0.5rem;
    color: white;
    display: flex;
    gap: 1.5rem;
    align-items: center;
    z-index: 100;
  }

  .track-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .title {
    font-weight: 600;
    font-size: 0.95rem;
  }

  .artist {
    font-size: 0.8rem;
    opacity: 0.8;
  }

  .controls {
    display: flex;
    gap: 0.5rem;
  }

  button {
    background: rgba(255, 255, 255, 0.2);
    border: none;
    color: white;
    width: 2rem;
    height: 2rem;
    border-radius: 50%;
    cursor: pointer;
    font-size: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  button:hover {
    background: rgba(255, 255, 255, 0.3);
  }
</style>
