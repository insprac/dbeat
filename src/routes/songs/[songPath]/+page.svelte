<script lang="ts">
    import Header from "../../../components/header.svelte";
    import { openFileLocation } from "../../../api";
    import { displayDuration } from "../../../time";
    import { Icon } from "../../../components/icons";
    import IconLabel from "../../../components/icon_label.svelte";

    export let data;
    const { song } = data;

    const displayPath = song.filePath.replace("/", "");
</script>

<Header
    tag="SONG"
    title={displayPath}
    onClick={() => openFileLocation(song.filePath)}
/>

<main>
    <h2 class="title">{song.title || "[Unknown]"}</h2>
    <p class="artist">{song.artist}</p>

    <div class="metadata-container">
        <IconLabel icon={Icon.Clock} tooltip="Duration">
            {displayDuration(song.durationSeconds)}
        </IconLabel>

        {#if song.album}
            <IconLabel icon={Icon.Album} tooltip="Album">
                {song.album}
            </IconLabel>
        {/if}

        {#if song.genre}
            <IconLabel icon={Icon.Genre} tooltip="Genre">
                {song.genre}
            </IconLabel>
        {/if}

        {#if song.bpm}
            <IconLabel icon={Icon.Bpm} tooltip="BPM">
                {song.bpm}
            </IconLabel>
        {/if}
    </div>
</main>

<style>
    main {
        margin-top: 1.5rem;
    }

    .artist {
        color: #999;
        margin-bottom: 1rem;
    }

    .metadata-container {
        display: flex;
        gap: 1rem;
        color: #999;
    }
</style>
