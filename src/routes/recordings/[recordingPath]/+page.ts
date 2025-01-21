import { getCueSheet, getDefaultPath } from "../../../api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
    const { recordingPath } = params;
    const path = decodeURIComponent(recordingPath);

    const cueSheet = await getCueSheet(path);

    const recordingDir = await getDefaultPath().catch(() => "");
    const truncatedFilePath = recordingDir
        ? cueSheet.filePath.replace(recordingDir, "...")
        : cueSheet.filePath;

    return { cueSheet, truncatedFilePath };
}
