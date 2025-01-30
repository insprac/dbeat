export interface Song {
    filePath: string;
    title?: string;
    artist?: string;
    album?: string;
    genre?: string;
    bpm?: string;
    durationSeconds: number;
}
