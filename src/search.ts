import type { CueSheet } from "./cue";

/**
 * Search a list of cue sheets using a search term, the search checks if any of the following:
 * - Recording title
 * - Recording DJ
 * - All track titles
 * - All track artist
 */
export function searchCueSheets(cueSheets: CueSheet[], searchTerm: string): CueSheet[] {
    searchTerm = searchTerm.trim().toLowerCase();

    let filtered = [];
    for (let sheet of cueSheets) {
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

