<script lang="ts">
  import { onMount } from 'svelte';

  let firstPaint = $state(0);
  let domInteractive = $state(0);

  onMount(() => {
    if (!window.performance) return;

    const checkStats = () => {
      // 1. Get First Contentful Paint (FCP)
      const paintEntries = window.performance.getEntriesByType('paint');
      const fcpEntry = paintEntries.find(entry => entry.name === 'first-contentful-paint');
      if (fcpEntry) {
        firstPaint = Math.round(fcpEntry.startTime);
      } else {
        // Fallback to domContentLoadedEventEnd
        const nav = window.performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
        if (nav && nav.domContentLoadedEventEnd > 0) {
          firstPaint = Math.round(nav.domContentLoadedEventEnd);
        }
      }

      // 2. Get DOM Interactive (DOM Ready)
      const navEntries = window.performance.getEntriesByType('navigation');
      if (navEntries.length > 0) {
        const nav = navEntries[0] as PerformanceNavigationTiming;
        if (nav && nav.domInteractive > 0) {
          domInteractive = Math.round(nav.domInteractive);
        }
      } else {
        const timing = window.performance.timing;
        if (timing && timing.navigationStart > 0 && timing.domInteractive > 0) {
          domInteractive = Math.round(timing.domInteractive - timing.navigationStart);
        }
      }
    };

    // Calculate once load event is fully completed and registered
    if (document.readyState === 'complete') {
      checkStats();
    } else {
      window.addEventListener('load', () => {
        setTimeout(checkStats, 100);
      });
    }
  });
</script>

<footer class="footer">
  <div class="content container">
    <div class="stats-wrapper">
      {#if firstPaint > 0}
        <span class="stat-item" title="Time to render first contentful pixels to screen">
          first paint <span class="value">{firstPaint}ms</span>
        </span>
      {/if}

      {#if firstPaint > 0 && domInteractive > 0}
        <span class="separator">·</span>
      {/if}

      {#if domInteractive > 0}
        <span class="stat-item" title="Time when DOM is fully loaded and interactive">
          dom ready <span class="value">{domInteractive}ms</span>
        </span>
      {/if}
    </div>
  </div>
</footer>

<style lang="sass">
  @use '../styles/variables' as *

  .footer
    background-color: $bg-footer
    padding: 1.25rem 1rem
    border-top: 1px solid $border-color

  .content
    display: flex
    justify-content: center
    align-items: center

  .stats-wrapper
    display: flex
    flex-wrap: wrap
    justify-content: center
    align-items: center
    gap: 0.25rem 0.5rem
    font-family: $font-family
    font-size: 0.8125rem
    color: $text-grey
    line-height: 1.4
    text-align: center

  .stat-item
    display: inline-flex
    align-items: center
    white-space: nowrap
    cursor: default

  .value
    font-weight: 550
    color: $text-dark
    margin-left: 0.15rem

  .separator
    color: $text-grey
    font-weight: 700
    user-select: none
    padding: 0 0.15rem
</style>
