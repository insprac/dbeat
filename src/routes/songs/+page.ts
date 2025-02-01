import { findSongs, getMusicDir } from "../../api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
    const songs = await findSongs();
    const musicDir = await getMusicDir();
    return { songs, musicDir };
}
