<script lang="ts">
    let {
        title,
        tag,
        onClick,
    }: { title: string; tag: string; onClick?: () => void } = $props();

    function back() {
        history.back();
    }
</script>

<header>
    <button class="back-button" aria-label="back" onclick={back}>
        <span class="arrow"></span>
    </button>
    <div class="route-bar">
        <div class="tag">{tag}</div>
        <button class="title" onclick={onClick}>{title}</button>
    </div>
</header>

<style>
    header {
        width: 100%;
        max-width: 100%;
        display: flex;
        gap: 1rem;
    }

    .back-button {
        width: 3rem;
        height: 3rem;
        flex-shrink: 0;
        font-size: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #222;
        transition: background-color 0.2s;
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

    .route-bar {
        width: calc(100% - 4rem);
        height: 3rem;
        padding: 0.1rem 1rem;
        flex-grow: 1;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background-color: #222;
        border-radius: 3rem;
    }

    .tag {
        color: #aaa;
        font-weight: 500;
    }

    .title {
        color: #999;
        width: 100%;
        text-align: left;
        direction: rtl;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        cursor: pointer;
        transition: color .2s;
    }

    .title:hover {
        color: #ccc;
    }
</style>
