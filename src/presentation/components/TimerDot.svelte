<script lang="ts">
  let { secondsLeft }: { secondsLeft: number } = $props();
</script>

<div class="timer-dot">
  {secondsLeft}
</div>

<style>
  .timer-dot {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    color: white;
    opacity: 0.2;
    font-size: 0.55rem;
    font-family: monospace;
    z-index: 10;
  }
</style>
