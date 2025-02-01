import { getRecording } from "../../../api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
    const { recordingPath } = params;
    const path = decodeURIComponent(recordingPath);

    const recording = await getRecording(path);

    return { recording };
}
