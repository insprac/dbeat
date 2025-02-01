<script lang="ts">
    import Header from "../../../components/header.svelte";
    import { openFileLocation } from "../../../api";
    import type { Track } from "../../../recording";
    import IconLabel from "../../../components/icon_label.svelte";
    import { Icon } from "../../../components/icons";
    import { displayDuration } from "../../../time";

    export let data;
    const { recording } = data;

    // There seems to be a bug in Safari WebKit where using `direction: rtl` causes the leading
    // slash to be appended instead, removing the leading slash is a fine work around for now.
    let displayPath = recording.filePath.replace("/", "");
    if (displayPath.endsWith(".cue")) {
        displayPath = displayPath.substring(0, displayPath.length - 4);
    }

    function generateTrackLink(track: Track): string | null {
        if (track.file) {
            return `/songs/${encodeURIComponent(track.file.name)}`;
        } else {
            return null;
        }
    }
</script>

<Header
    title={displayPath}
    tag="RECORDING"
    onClick={() => openFileLocation(recording.filePath)}
/>

<main>
    <div>
        <h2>{recording.title}</h2>
        <p class="artist">{recording.performer}</p>
    </div>
    {#if recording.waveFile}
        <div class="metadata">
            <IconLabel icon={Icon.Clock} tooltip="Duration">
                {displayDuration(recording.waveFile.durationSeconds)}
            </IconLabel>
        </div>
    {/if}
    <div class="track-list">
        {#each recording.tracks as track}
            <p>
                <span class="start-time">{track.startTime}</span>
                <a href={generateTrackLink(track)} class="track-link">
                    {track.title}
                </a>
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

    .artist {
        color: #999;
    }

    .metadata {
        color: #aaa;
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
