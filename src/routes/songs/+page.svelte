<script lang="ts">
    import type { PageProps } from "../$types";
    import Header from "../../components/header.svelte";
    import TextInput from "../../components/inputs/text_input.svelte";
    import { searchSongs } from "../../search";
    import { displayDuration } from "../../time";

    let { data }: PageProps = $props();

    let searchTerm = $state("");

    function onSearchInput(value: string) {
        searchTerm = value;
    }
</script>

<main>
    <Header tag="SONGS" title={data.musicDir} />

    <TextInput
        value={searchTerm}
        onInput={onSearchInput}
        placeholder="Search..."
    />

    <table>
        <thead>
            <tr>
                <th>Title</th>
                <th>Artist</th>
                <th>Album</th>
                <th>Duration</th>
            </tr>
        </thead>
        <tbody>
            {#each searchSongs(data.songs, searchTerm || "") as song}
                <tr>
                    <td class="title">
                        <a href={`/songs/${encodeURIComponent(song.filePath)}`}>
                            {song.title}
                        </a>
                    </td>
                    <td class="artist">{song.artist}</td>
                    <td class="album">{song.album}</td>
                    <td class="duration">
                        {displayDuration(song.durationSeconds || 0)}
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        color: #aaaaaa;
    }

    input {
        width: 100%;
        padding: 0.4rem 0.8rem;
        background-color: #333;
        border-radius: 4px;
        border: none;
        font-size: 16px;
        color: white;
    }

    input:focus {
        outline: 2px solid white;
    }

    th,
    td {
        text-align: left;
        overflow-x: auto;
        padding: 0.2rem 0.2rem;
    }

    td.title {
        color: #eee;
    }

    td.artist {
        color: #777;
    }

    td.album {
        color: #777;
    }

    td.duration {
        color: #aaa;
    }

    a {
        color: #eee;
        text-decoration: none;
    }

    a:hover {
        color: #fff;
    }
</style>
