<script lang="ts">
    import { getDefaultPath } from "../../api";

    let recordingPath = $state(null as null | string);

    getDefaultPath()
        .then((path) => {
            console.log("path", path);
            recordingPath = path;
        })
        .catch(console.error);
</script>

<main>
    <p>Path: {recordingPath}</p>
</main>

<style>
    main {
        padding: 1rem;
        color: #aaaaaa;
    }
</style>
