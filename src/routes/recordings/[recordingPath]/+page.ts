import { getCueSheet } from "../../../api";
import type { CueSheet } from "../../../cue";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
    const { recordingPath } = params;
    const path = decodeURIComponent(recordingPath);

    const cueSheet = await getCueSheet(path);

    return { cueSheet };
}
