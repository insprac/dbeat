<script lang="ts">
    const { children } = $props();
</script>

<div class="layout-container">
    <div class="sidebar">
        <nav>
            <a href="/recordings">Recordings</a>
            <a href="/songs">Songs</a>
            <a href="/settings">Settings</a>
        </nav>
    </div>

    <div class="body">
        {@render children()}
    </div>
</div>

<style>
    .layout-container {
        max-width: 100%;
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
        font-size: 18px;
        gap: 0.5rem;
    }

    nav a {
        display: block;
        text-decoration: none;
        color: #aaaaaa;
        font-weight: 500;
        transition: color .2s;
    }

    nav a:hover {
        color: #ffffff;
    }

    .body {
        width: calc(100vw - 200px);
        padding: 1rem;
        overflow-x: hidden;
        overflow-y: auto;
    }
</style>
