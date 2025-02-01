export interface Recording {
  filePath: string;
  rem: [string, string][];
  title?: string;
  performer?: string;
  file?: File;
  tracks: Track[];
  waveFile?: WaveFile;
}

export interface File {
  name: string;
  format: string;
}

export interface Track {
  title?: string;
  performer?: string;
  file?: File;
  startTime?: string;
}

export interface WaveFile {
    filePath: string;
    channels: string;
    sampleRate: number;
    bitsPerSample: number;
    sampleFormat: string;
    durationSeconds: number;
    totalSamples: number;
}
