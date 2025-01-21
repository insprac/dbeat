<script lang="ts">
    import Header from "../../../components/header.svelte";
    import { openFileLocation } from "../../../api";

    export let data;
    const { cueSheet } = data;

    // There seems to be a bug in Safari WebKit where using `direction: rtl` causes the leading
    // slash to be appended instead, removing the leading slash is a fine work around for now.
    let displayPath = cueSheet.filePath.replace("/", "");
    if (displayPath.endsWith(".cue")) {
        displayPath = displayPath.substring(0, displayPath.length - 4);
    }
</script>

<Header
    title={displayPath}
    tag="RECORDING"
    onClick={() => openFileLocation(cueSheet.filePath)}
/>

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
        margin-top: 1rem;
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
