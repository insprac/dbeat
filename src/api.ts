import { invoke } from "@tauri-apps/api/core";
import type { CueSheet } from "./cue";
import type { Song } from "./song";

export async function getDefaultPath(): Promise<string | null> {
    return await invoke("get_default_path");
}

export async function getCueSheet(path: string): Promise<CueSheet> {
    return await invoke("get_cue_sheet", { path });
}

export async function findCueSheets(): Promise<CueSheet[]> {
    return await invoke("find_cue_sheets", {
        dir: "/Users/insprac/Music/PioneerDJ/Recording",
    });
}

export async function openFileLocation(path: string): Promise<null> {
    return await invoke("open_file_location", { path });
}

export async function getSong(path: string): Promise<Song> {
    return await invoke("get_song", { path });
}

