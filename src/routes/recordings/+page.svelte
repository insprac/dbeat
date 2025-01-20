<script lang="ts">
    import { findCueSheets } from "../../api";
    import type { CueSheet } from "../../cue";

    let cueSheets = $state([] as CueSheet[]);
    let error = $state(null as string | null);

    findCueSheets()
        .then((sheets) => (cueSheets = sheets))
        .catch(console.error);
</script>

<main>
    {#if error}
        <p class="error">{error}</p>
    {:else}
        {#each cueSheets as sheet}
            <p>{sheet.title} ({sheet.tracks.length} songs)</p>
        {/each}
    {/if}
</main>

<style>
    main {
        padding: 1rem;
        color: #aaaaaa;
    }
</style>
