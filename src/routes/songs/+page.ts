import { findSongs } from "../../api";
import type { Song } from "../../song";
import type { PageLoad } from "./$types";

export const load: PageLoad = async (): Promise<{ songs: Song[] }> => {
    const songs = await findSongs();
    return { songs };
}
