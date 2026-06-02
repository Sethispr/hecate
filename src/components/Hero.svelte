<script lang="ts">
  import { store } from '../store.svelte.ts';
</script>

<section class="hero is-{store.overallStatus}">
  <div class="hero-body">
    <div class="container">
      <h1 class="title">blair.</h1>
      <p class="subtitle">
        {store.statusMessage}
      </p>
    </div>
  </div>
</section>

<style lang="sass">
  @use '../styles/variables' as *

  .hero
    transition: background-color 0.25s cubic-bezier(0.4, 0, 0.2, 1)
    will-change: background-color
    &.is-success
      background-color: #2b9e66
      .title, .subtitle
        color: #ffffff
    &.is-warning
      background-color: #ffe08a
      .title, .subtitle
        color: rgba(0, 0, 0, 0.85)
    &.is-danger
      background-color: #d12e52
      .title, .subtitle
        color: #ffffff

  .hero-body
    padding: 3rem 0 4rem 0
    @media (min-width: 768px)
      padding: 4rem 0 5rem 0

  .container
    width: 100%
    max-width: 1024px
    margin: 0 auto
    padding: 0 1.25rem
    @media (min-width: 768px)
      padding: 0 1.5rem

  .title
    font-size: 2.5rem
    font-weight: 700
    letter-spacing: -1px
    margin-bottom: 0.5rem
    font-family: 'Fira Sans', sans-serif
    @media (min-width: 768px)
      font-size: 3.5rem

  .subtitle
    font-size: 1.25rem
    font-weight: 400
    line-height: 1.3
    @media (min-width: 768px)
      font-size: 1.5rem
</style>
