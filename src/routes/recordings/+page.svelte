<script lang="ts">
    import { findCueSheets } from "../../api";
    import type { CueSheet } from "../../cue";
    import { displayDuration } from "../../time";

    let cueSheets = $state([] as CueSheet[]);

    findCueSheets()
        .then((sheets) => (cueSheets = sheets))
        .catch(console.error);
</script>

<main>
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
            {#each cueSheets as sheet}
                <tr>
                    <td class="title">
                        <a href={`/mixes/${encodeURIComponent(sheet.filePath)}`}>
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
        padding: 1rem;
        color: #aaaaaa;
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
