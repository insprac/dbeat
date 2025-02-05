import type { Recording } from "./recording";
import type { Song } from "./song";

/**
 * Search a list of recordings using a search term, the search checks if any of the following:
 * - Recording title
 * - Recording DJ
 * - All track titles
 * - All track artist
 */
export function searchRecordings(recordings: Recording[], searchTerm: string): Recording[] {
    searchTerm = searchTerm.trim().toLowerCase();

    let filtered = [];
    for (let sheet of recordings) {
        if (
            !searchTerm ||
            // Search mix title and DJ
            sheet.title?.toLowerCase().includes(searchTerm) ||
            sheet.performer?.toLowerCase().includes(searchTerm) ||
            // Check each track in the mix
            sheet.tracks.find((track) => {
                return (
                    track.title?.toLowerCase().includes(searchTerm) ||
                    track.performer?.toLowerCase().includes(searchTerm)
                );
            })
        ) {
            filtered.push(sheet);
        }
    }

    return filtered;
}


/**
 * Search a list of songs using a search term, the search checks if any of the following:
 * - Title
 * - Artist
 * - Album
 * - Genre
 */
export function searchSongs(songs: Song[], searchTerm: string): Song[] {
    searchTerm = searchTerm.trim().toLowerCase();

    let filtered = [];
    for (let song of songs) {
        if (
            !searchTerm ||
            song.title?.toLowerCase().includes(searchTerm) ||
            song.artist?.toLowerCase().includes(searchTerm) ||
            song.album?.toLowerCase().includes(searchTerm) ||
            song.genre?.toLowerCase().includes(searchTerm)
        ) {
            filtered.push(song);
        }
    }

    return filtered;
}

