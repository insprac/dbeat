import { findRecordings, getRecordingsDir } from "../../api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
    const recordings = await findRecordings();
    const recordingsDir = await getRecordingsDir();
    return { recordings, recordingsDir };
}
