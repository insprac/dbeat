import { getSong } from "../../../api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
    const { songPath } = params;
    const path = decodeURIComponent(songPath);
    return { song: await getSong(path) };
}
