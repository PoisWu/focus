<script lang="ts">
  import { fade } from "svelte/transition";
  import type { Photo } from "../../domain/photo";

  let { photo }: { photo: Photo } = $props();
</script>

{#key photo.id}
  <div class="photo-frame" transition:fade={{ duration: 800 }}>
    <img src={photo.url} alt={photo.photographer} />
  </div>
{/key}

<style>
  .photo-frame {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 0;
  }

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
</style>
