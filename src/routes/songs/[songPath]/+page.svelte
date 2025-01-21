<script lang="ts">
    import Header from "../../../components/header.svelte";
    import { openFileLocation } from "../../../api";
    import { displayDuration } from "../../../time";

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
    <h3 class="title">{song.title || "[Unknown]"}</h3>
    <p>Artist: {song.artist}</p>
    <p>Duration: {displayDuration(song.durationSeconds)}</p>
    <p>Album: {song.album || "-"}</p>
    <p>Genre: {song.genre || "-"}</p>
</main>

<style>
    main {
        margin-top: 1rem;
    }
</style>
