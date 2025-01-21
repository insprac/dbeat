<script lang="ts">
    import Header from "../../../components/header.svelte";
    import { openFileLocation } from "../../../api";
    import type { Track } from "../../../cue";

    export let data;
    const { cueSheet } = data;

    // There seems to be a bug in Safari WebKit where using `direction: rtl` causes the leading
    // slash to be appended instead, removing the leading slash is a fine work around for now.
    let displayPath = cueSheet.filePath.replace("/", "");
    if (displayPath.endsWith(".cue")) {
        displayPath = displayPath.substring(0, displayPath.length - 4);
    }

    function generateTrackLink(track: Track): string | null {
        if (track.file) {
            return `/songs/${encodeURIComponent(track.file.name)}`
        } else {
            return null
        }
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
                <a href={generateTrackLink(track)} class="track-link">{track.title}</a>
                <span class="performer">- {track.performer}</span>
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

    .track-link {
        color: #eee;
        text-decoration: none;
        cursor: pointer;
    }

    .track-link:hover {
        color: white;
    }

    .performer {
        color: #777;
    }
</style>
