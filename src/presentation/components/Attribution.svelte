<script lang="ts">
  import type { Photo } from "../../domain/photo";

  let { photo }: { photo: Photo } = $props();
</script>

<div class="attribution">
  Photo by <a href={photo.profileUrl} target="_blank" rel="noopener noreferrer">
    {photo.photographer}
  </a> on Unsplash
</div>

<style>
  .attribution {
    position: fixed;
    bottom: 1rem;
    left: 1rem;
    color: white;
    opacity: 0.4;
    font-size: 0.75rem;
    z-index: 10;
  }

  .attribution:hover {
    opacity: 0.8;
  }

  a {
    color: white;
    text-decoration: none;
  }

  a:hover {
    text-decoration: underline;
  }
</style>
