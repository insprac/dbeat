import { findCueSheets } from "../../api";
import type { CueSheet } from "../../cue";
import type { PageLoad } from "./$types";

export const load: PageLoad = async (): Promise<{ cueSheets: CueSheet[] }> => {
    const cueSheets = await findCueSheets();
    return { cueSheets };
}
