<nav class="navbar">
  <div class="container">
    <a href="/" class="brand">uptime.rs</a>
  </div>
</nav>

<style lang="sass">
  @use '../styles/variables' as *

  .navbar
    background-color: $bg-color
    border-bottom: 1px solid $border-color
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05)

  .container
    display: flex
    align-items: center
    height: 3.25rem

  .brand
    font-size: 1.5rem
    font-weight: 500
    color: $text-dark
    letter-spacing: -0.5px
    &:hover
      text-decoration: none
</style>
