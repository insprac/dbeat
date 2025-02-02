//! Provides cue sheet (.cue) file support for parsing and metadata extraction.
//! Support only extends to the fields Rekordbox exports, any other properties that can be in cue
//! sheets are ignored for now, further support may be added in future.

use serde::{Deserialize, Serialize};

/// Track metadata extracted from a cue sheet representing a song in the recording.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    /// The name of the song.
    pub title: Option<String>,
    /// The song's artist.
    pub performer: Option<String>,
    /// The local file where the song is located.
    pub file: Option<File>,
    /// The time which the track starts playing in the recording, format 'HH:MM:SS', e.g. '00:24:36'.
    pub start_time: Option<String>,
}

/// File metadata extracted from a cue sheet pointing to a local file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub name: String,
    pub format: String,
}

/// Data extracted from a wave (.wav) file associated with a mix, necessary for getting the
/// duration and other potentially useful info to display.
///
/// Wave files are found by looking for a file in the same directory and with the same name as the
/// cue sheet being parsed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveFile {
    /// The origin wave file path where this data was extracted from.
    pub file_path: String,
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub sample_format: String,
    pub duration_seconds: f64,
    pub total_samples: u32,
}

/// Represents a recording/mix from Rekordbox, instead of supporting all cue sheet syntax we only
/// support the fields Rekordbox exports.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recording {
    /// The original cue sheet file path where this data was extracted from.
    pub file_path: String,
    /// Last modified (unix seconds) taken from the file's metadata.
    pub last_modified_unix_seconds: i64,
    /// Last accessed (unix seconds) taken from the file's metadata.
    pub last_accessed_unix_seconds: i64,
    /// A list of metadata comments.
    pub rem: Vec<(String, String)>,
    /// The title of the recording.
    pub title: Option<String>,
    /// The DJ who created the recording.
    pub performer: Option<String>,
    /// File metadata, this will reference the actual wave file but we currently use other methods
    /// of finding the wave file.
    pub file: Option<File>,
    /// Any number of tracks that were used in the recording including their start timestamp.
    pub tracks: Vec<Track>,
    /// Extracted metadata from the wave file (if it was found).
    pub wave_file: Option<WaveFile>,
}

impl Recording {
    pub fn parse(file_path: &str, input: &str) -> Self {
        let mut recording = Self {
            file_path: file_path.to_string(),
            ..Default::default()
        };

        // Try get the last modified and access times from the file's metadata, ignore any errors.
        if let Ok(metadata) = std::fs::metadata(file_path) {
            let modified_time = filetime::FileTime::from_last_modification_time(&metadata);
            recording.last_modified_unix_seconds = modified_time.unix_seconds();

            let accessed_time = filetime::FileTime::from_last_access_time(&metadata);
            recording.last_accessed_unix_seconds = accessed_time.unix_seconds();
        }

        let mut current_track: Option<Track> = None;

        for line in input.lines() {
            let tab_count = line.chars().take_while(|&c| c == '\t').count();
            let line = line.trim();
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 0 {
                continue;
            }

            match parts[0] {
                // REM is like a comment containing metadata, we store it as a key value pair.
                "REM" => {
                    if parts.len() >= 3 {
                        let key = parts[1].to_string();
                        let value = parts[2..].join(" ").trim_matches('"').to_string();
                        recording.rem.push((key, value));
                    }
                }
                // TITLE can be part of the entire recording or a TRACK.
                "TITLE" => {
                    if let Some(title) = extract_quoted_string(&line[5..]) {
                        if let Some(track) = &mut current_track {
                            track.title = Some(title);
                        } else if tab_count == 0 {
                            recording.title = Some(title);
                        }
                    }
                }
                // PERFORMER can be part of the entire recording or a TRACK.
                "PERFORMER" => {
                    if let Some(performer) = extract_quoted_string(&line[9..]) {
                        if let Some(track) = &mut current_track {
                            track.performer = Some(performer);
                        } else if tab_count == 0 {
                            recording.performer = Some(performer);
                        }
                    }
                }
                // FILE can be part of the entire recording or a TRACK.
                "FILE" => {
                    if let Some(format) = parts.last() {
                        let line_inner = &line[4..line.len() - format.len()];
                        if let Some(name) = extract_quoted_string(line_inner) {
                            let file = File {
                                name,
                                format: format.to_string(),
                            };
                            if let Some(track) = &mut current_track {
                                track.file = Some(file);
                            } else if tab_count == 0 {
                                recording.file = Some(file);
                            }
                        }
                    }
                }
                "TRACK" => {
                    if let Some(track) = &mut current_track {
                        recording.tracks.push(track.to_owned());
                    }
                    current_track = Some(Track::default());
                }
                "INDEX" => {
                    if let Some(start_time) = parts.get(2) {
                        if let Some(track) = &mut current_track {
                            track.start_time = Some(start_time.to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(track) = current_track {
            recording.tracks.push(track);
        }

        recording
    }
}

/// Extracts the text between two quotes, falling back to the whole string
fn extract_quoted_string(line: &str) -> Option<String> {
    if let Some(start) = line.find('"') {
        if let Some(end) = line[start + 1..].find('"') {
            return Some(line[start + 1..start + 1 + end].to_string());
        }
    }

    // Fallback for unquoted strings: take whole string
    Some(line.trim().to_string())
}
