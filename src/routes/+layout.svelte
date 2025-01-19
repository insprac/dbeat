<script lang="ts">
    const { children } = $props();
</script>

<div class="layout-container">
    <div class="sidebar">
        <nav>
            <a href="/mixes">Mixes</a>
        </nav>
    </div>

    <main>
        {@render children()}
    </main>
</div>

<style>
    .layout-container {
        display: flex;
    }

    .sidebar {
        position: sticky;
        top: 0;
        width: 200px;
        min-width: 200px;
        height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #1f1f1f;
    }

    nav {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    nav a {
        display: block;
        text-transform: uppercase;
        text-decoration: none;
        color: #aaaaaa;
        font-weight: 500;
        transition: color .2s;
    }

    nav a:hover {
        color: #ffffff;
    }

    main {
        flex-grow: 1;
        overflow: auto;
    }
</style>
