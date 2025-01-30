<script lang="ts">
    import Header from "../../../components/header.svelte";
    import { openFileLocation } from "../../../api";
    import { displayDuration } from "../../../time";
    import { Icon } from "../../../components/icons";
    import Metadata from "../../../components/metadata.svelte";

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
        <Metadata icon={Icon.Clock}>
            {displayDuration(song.durationSeconds)}
        </Metadata>

        {#if song.album}
            <Metadata icon={Icon.Album}>
                {song.album}
            </Metadata>
        {/if}

        {#if song.genre}
            <Metadata icon={Icon.Genre}>
                {song.genre}
            </Metadata>
        {/if}

        {#if song.bpm}
            <Metadata icon={Icon.Bpm}>
                {song.bpm}
            </Metadata>
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
        flex-direction: column;
        gap: .5rem;
    }
</style>
