import { findCueSheets, getMusicDir, getRecordingsDir } from "../../api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
    const cueSheets = await findCueSheets();
    const recordingsDir = await getRecordingsDir();
    return { cueSheets, recordingsDir };
}
