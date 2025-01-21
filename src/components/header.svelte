<script lang="ts">
    let { title, tag }: { title: string; tag: string; } = $props();
    function back() {
        history.back();
    }
</script>

<header>
    <button class="back-button" aria-label="back" onclick={back}>
        <span class="arrow"></span>
    </button>
    <div class="route">
        <div class="tag">{tag}</div>
        <div class="title">{title}</div>
    </div>
</header>

<style>
    header {
        display: flex;
        gap: 1rem;
    }

    .back-button {
        width: 3rem;
        height: 3rem;
        font-size: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #222;
        transition: background-color .2s;
        border-radius: 100%;
        cursor: pointer;
    }

    .back-button:hover {
        background-color: #444;
    }

    .back-button .arrow {
        display: block;
        width: 0.8rem;
        height: 0.8rem;
        transform: translateX(0.15rem) rotate(45deg);
        border: 0.2rem solid #aaa;
        border-top: none;
        border-right: none;
    }

    .route {
        height: 3rem;
        padding: 0.1rem 1rem;
        flex-grow: 1;
        display: flex;
        align-items: center;
        gap: .5rem;
        background-color: #222;
        border-radius: 3rem;
    }

    .tag {
        color: #aaa;
        font-weight: 500;
    }

    .title {
        color: #999;
    }
</style>
