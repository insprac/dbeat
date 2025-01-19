import { invoke } from "@tauri-apps/api/core";
import type { CueSheet } from "./cue";

export async function getDefaultPath(): Promise<string> {
    return await invoke("get_default_path");
}

export async function findCueSheets(): Promise<CueSheet[]> {
    return await invoke("find_cue_sheets", {
        dir: "/Users/insprac/Music/PioneerDJ/Recording",
    });
}
