import { getCueSheet, getDefaultPath, getSong } from "../../../api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
    const { songPath } = params;
    const path = decodeURIComponent(songPath);

    const song = await getSong(path);

    return { song };
}
