<script lang="ts">
    import type { PageProps } from "../$types";
    import type { Song } from "../../song";
    import { searchSongs } from "../../search";
    import { displayDuration } from "../../time";

    let { data }: PageProps = $props();

    let search = $state("");

    function onSearchInput(event: Event) {
        if (event.target) {
            search = (event.target as HTMLInputElement).value;
        }
    }
</script>

<main>
    <input type="text" oninput={onSearchInput} placeholder="Search..." />
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
            {#each searchSongs(data.songs, search || "") as song}
                <tr>
                    <td class="title">
                        <a
                            href={`/songs/${encodeURIComponent(song.filePath)}`}
                        >
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
        margin-bottom: 1rem;
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

    td.performer {
        color: #777;
    }

    td.tracks {
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
