use std::path::Path;

use crate::cue::{CueSheet, WaveFile};

/// Searches the given directory for all cue sheet (.cue) files and associated (.wav) file.
/// Cue sheets and wave files are parsed extracting their metadata.
///
/// Rekordbox recordings are exported in their own directory including both a (.cue) and (.wav)
/// file with the same filename aside from the extension.
///
/// Any IO errors are logged and otherwise ignored.
pub fn find_cue_sheets(path: &str) -> Vec<CueSheet> {
    let dir = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(error) => {
            tracing::warn!(?error, "failed to read directory");
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

    let mut cue_sheets: Vec<CueSheet> = Vec::new();

    for file in cue_files {
        match std::fs::read_to_string(&file) {
            Ok(content) => {
                let mut cue = CueSheet::parse(file, &content);
                cue.wave_file = try_find_wav_for_cue_file(&cue.file_path);
                cue_sheets.push(cue);
            }
            Err(error) => {
                tracing::warn!(?error, file, "failed to read cue file");
            }
        }
    }

    for dir in dirs {
        cue_sheets.append(&mut find_cue_sheets(&dir));
    }

    cue_sheets
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
