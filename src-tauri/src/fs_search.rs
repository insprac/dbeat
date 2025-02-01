use std::path::Path;

use lofty::file::FileType;

use crate::{
    recording::{Recording, WaveFile},
    songs::Song,
};

/// Searches the given directory for all cue sheet (.cue) files and associated (.wav) file.
/// Cue sheets and wave files are parsed extracting their metadata.
///
/// Rekordbox recordings are exported in their own directory including both a (.cue) and (.wav)
/// file with the same filename aside from the extension.
///
/// Any IO errors are logged and otherwise ignored.
pub fn find_recordings(path: &str) -> Vec<Recording> {
    let dir = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(error) => {
            tracing::warn!(?error, path, "failed to read directory");
            return Vec::new();
        }
    };

    // TODO make this async

    let mut dirs: Vec<String> = Vec::new();
    let mut cue_files: Vec<String> = Vec::new();

    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                tracing::warn!(?error, "failed to read entry in dir");
                continue;
            }
        };

        let entry_path = entry.path();
        let Some(entry_path_str) = entry_path.to_str().map(|str| str.to_string()) else {
            continue;
        };

        if entry_path.is_dir() {
            dirs.push(entry_path_str);
        } else if entry_path.extension().and_then(|ext| ext.to_str()) == Some("cue") {
            cue_files.push(entry_path_str);
        }
    }

    let mut recordings: Vec<Recording> = Vec::new();

    for file in cue_files {
        match read_recording(&file) {
            Ok(recording) => {
                recordings.push(recording);
            }
            Err(error) => {
                tracing::warn!(?error, file, "failed to read cue file");
            }
        }
    }

    for dir in dirs {
        recordings.append(&mut find_recordings(&dir));
    }

    recordings
}

pub fn find_songs(path: &str) -> Vec<Song> {
    let dir = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(error) => {
            tracing::warn!(?error, path, "failed to read directory");
            return Vec::new();
        }
    };

    let mut dirs: Vec<String> = Vec::new();
    let mut song_files: Vec<String> = Vec::new();

    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                tracing::warn!(?error, "failed to read entry in dir");
                continue;
            }
        };

        let entry_path = entry.path();
        let Some(entry_path_str) = entry_path.to_str().map(|str| str.to_string()) else {
            continue;
        };

        if entry_path.is_dir() {
            dirs.push(entry_path_str);
        } else if entry_path
            .extension()
            // Check if the file type is an audio file supported by `lofty`.
            .and_then(|ext| FileType::from_ext(ext))
            .is_some()
        {
            song_files.push(entry_path_str);
        }
    }

    let mut songs: Vec<Song> = Vec::new();

    for file_path in song_files {
        match Song::from_file(&file_path).map_err(|e| e.to_string()) {
            Ok(song) => {
                songs.push(song);
            }
            Err(error) => {
                tracing::warn!(?error, file_path, "failed to read song file");
            }
        }
    }

    for dir in dirs {
        songs.append(&mut find_songs(&dir));
    }

    songs
}

/// Reads a cue sheet with the given path and parses the file contents.
/// Note this can result in an empty `Recording` struct if it's not formatted correctly.
/// Fails if there was an IO error such as the file not existing or lack of permission.
pub fn read_recording(path: &str) -> Result<Recording, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    let mut cue = Recording::parse(path.to_string(), &content);
    cue.wave_file = try_find_wav_for_cue_file(&cue.file_path);
    Ok(cue)
}

/// Trys to read and parse a (.wav) file with the same name as the given (.cue) file changing the
/// extention.
///
/// Any IO errors are logged and otherwise ignored.
fn try_find_wav_for_cue_file(cue_file_path: &str) -> Option<WaveFile> {
    let wave_path = Path::new(cue_file_path);
    let wave_path = wave_path.with_extension("wav");
    let wave_path_string = wave_path.to_string_lossy().to_string();

    match hound::WavReader::open(wave_path) {
        Ok(reader) => {
            let spec = reader.spec();
            Some(WaveFile {
                file_path: wave_path_string,
                channels: spec.channels,
                sample_rate: spec.sample_rate,
                bits_per_sample: spec.bits_per_sample,
                sample_format: format!("{:?}", spec.sample_format),
                duration_seconds: reader.duration() as f64 / spec.sample_rate as f64,
                total_samples: reader.duration(),
            })
        }
        Err(error) => {
            tracing::warn!(?error, file = wave_path_string, "failed to read wave file");
            None
        }
    }
}
