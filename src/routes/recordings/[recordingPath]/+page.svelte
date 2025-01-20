<script lang="ts">
    export let data;
    const { cueSheet } = data;
</script>

<main>
    <h3 class="title">{cueSheet.title}</h3>
    <p class="file-path">{cueSheet.filePath}</p>
    <div class="divider"></div>
    <div class="track-list">
        {#each cueSheet.tracks as track}
            <p>
                <span class="start-time">{track.startTime}</span>
                {track.title} <span class="performer">- {track.performer}</span>
            </p>
        {/each}
    </div>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .divider {
        border-bottom: 0.1rem solid #444;
    }

    .file-path {
        color: #888;
    }

    .start-time {
        color: #999;
    }

    .performer {
        color: #777;
    }
</style>
