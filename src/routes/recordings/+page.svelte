<script lang="ts">
    import type { PageProps } from "../$types";
    import Header from "../../components/header.svelte";
    import { searchRecordings } from "../../search";
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
    <Header tag="RECORDINGS" title={data.recordingsDir} />
    <input type="text" oninput={onSearchInput} placeholder="Search..." />
    <table>
        <thead>
            <tr>
                <th>Recording Name</th>
                <th>DJ</th>
                <th>Songs</th>
                <th>Duration</th>
            </tr>
        </thead>
        <tbody>
            {#each searchRecordings(data.recordings, search || "") as sheet}
                <tr>
                    <td class="title">
                        <a
                            href={`/recordings/${encodeURIComponent(sheet.filePath)}`}
                        >
                            {sheet.title}
                        </a>
                    </td>
                    <td class="performer">{sheet.performer}</td>
                    <td class="tracks">{sheet.tracks.length}</td>
                    <td class="duration">
                        {displayDuration(sheet.waveFile?.durationSeconds || 0)}
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</main>

<style>
    main {
        color: #aaaaaa;
        display: flex;
        flex-direction: column;
        gap: 1rem;
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
