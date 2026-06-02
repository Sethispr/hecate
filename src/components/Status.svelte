<script lang="ts">
  import { store } from '../store.svelte.ts';

  const offlineMainCount = $derived(store.mainServices.filter((s) => s.status !== 'online').length);
</script>

{#if offlineMainCount > 0}
  <div class="notification is-warning" role="alert">
    <p>Blair has <b>{offlineMainCount} main {offlineMainCount === 1 ? 'service' : 'services'} down</b>.</p>
  </div>
{/if}

<div class="grid margin-bottom">
  <div class="column">
    <h2 class="title is-3" id="main-services-heading">Main services</h2>
    <div class="table-container">
      <table class="table is-fullwidth is-striped" aria-labelledby="main-services-heading">
        <thead>
          <tr>
            <th scope="col">Service</th>
            <th scope="col" class="right">Status</th>
          </tr>
        </thead>
        <tbody>
          {#each store.mainServices as svc (svc.name)}
            <tr>
              <td>
                <a href="#{svc.name}">{svc.name}</a>
                {#if svc.metrics}
                  <div class="metrics">
                    <small>
                      Uptime: {svc.metrics.service?.uptime_seconds}s | Requests: {svc.metrics.requests
                        ?.total} | Cache Hit Rate: {svc.metrics.cache?.hit_rate_percent?.toFixed(1)}%
                    </small>
                  </div>
                {/if}
              </td>
              <td class="right">
                <span class="tag is-{svc.status === 'online' ? 'success' : 'danger'}" role="status">
                  {#if svc.status === 'online'}
                    <svg class="tag-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" width="12" height="12">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                    </svg>
                  {:else}
                    <svg class="tag-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" width="12" height="12">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                    </svg>
                  {/if}
                  {svc.status}
                </span>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  <div class="column">
    <h2 class="title is-3" id="other-services-heading">Other services</h2>
    <div class="table-container">
      <table class="table is-fullwidth is-striped" aria-labelledby="other-services-heading">
        <thead>
          <tr>
            <th scope="col">Service</th>
            <th scope="col" class="right">Status</th>
          </tr>
        </thead>
        <tbody>
          {#each store.otherServices as svc (svc.id || svc.name)}
            <tr>
              <td>
                {#if svc.id === 'discord'}
                  <a href="https://discordstatus.com" target="_blank" rel="noopener noreferrer" aria-label="{svc.name} (opens in a new tab)">{svc.name}</a>
                {:else if svc.id === 'topgg'}
                  <a href="https://status.top.gg" target="_blank" rel="noopener noreferrer" aria-label="{svc.name} (opens in a new tab)">{svc.name}</a>
                {:else}
                  <a href="#{svc.id || svc.name}">{svc.name}</a>
                {/if}
                {#if svc.description}
                  <div class="metrics">
                    <small>{svc.description}</small>
                  </div>
                {/if}
              </td>
              <td class="right">
                <span class="tag is-{svc.status === 'online' ? 'success' : 'danger'}" role="status">
                  {#if svc.status === 'online'}
                    <svg class="tag-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" width="12" height="12">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                    </svg>
                  {:else}
                    <svg class="tag-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" width="12" height="12">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                    </svg>
                  {/if}
                  {svc.status}
                </span>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>

<style lang="sass">
  @use '../styles/variables' as *

  .notification
    background-color: #f5f5f5
    border-radius: 4px
    position: relative
    padding: 1.25rem 2.5rem 1.25rem 1.5rem
    margin-bottom: 2rem
    color: #363636
    p
      margin: 0
    b
      color: currentColor
    &.is-warning
      background-color: #ffe08a
      color: rgba(0, 0, 0, 0.7)

  .grid
    display: grid
    grid-template-columns: 1fr
    gap: 2.5rem
    @media (min-width: 768px)
      grid-template-columns: 1fr 1fr

  .margin-bottom
    margin-bottom: 3rem

  .title.is-3
    font-size: 1.75rem
    color: $text-dark
    margin-bottom: 1.5rem
    font-weight: 500
    letter-spacing: -0.5px
    font-family: 'Fira Sans', sans-serif

  .table-container
    width: 100%
    overflow-x: auto
    -webkit-overflow-scrolling: touch
    margin-bottom: 1.5rem

  .table
    width: 100%
    border-collapse: collapse
    th
      font-size: 0.875rem
      font-weight: 600
      text-transform: capitalize
      color: $text-grey
      padding: 0.75rem 0.5rem
      border-bottom: 2px solid $border-color
      text-align: left
    td
      padding: 0.75rem 0.5rem
      border-bottom: 1px solid $border-color
      vertical-align: middle
    tbody tr
      transition: background-color 0.15s ease
      &:nth-child(even)
        background-color: #fafafa
      &:hover
        background-color: #f5f5f5

  .right
    text-align: right !important

  .metrics
    margin-top: 0.25rem
    color: $text-grey
    font-size: 0.85rem

  .tag
    align-items: center
    border-radius: 4px
    display: inline-flex
    font-size: 0.75rem
    font-weight: 500
    height: 1.8em
    justify-content: center
    line-height: 1.5
    padding-left: 0.75rem
    padding-right: 0.75rem
    white-space: nowrap
    text-transform: lowercase
    gap: 0.35rem
    &.is-success
      background-color: #2b9e66
      color: #ffffff
    &.is-danger
      background-color: #d12e52
      color: #ffffff

  .tag-icon
    display: inline-block
    flex-shrink: 0
</style>
