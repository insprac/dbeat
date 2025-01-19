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

/// Represents a recording/mix from Rekordbox, it's really just a cue sheet but we only care about
/// how Rekordbox exports them rather than supporting all cue sheet syntax.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CueSheet {
    /// The original cue sheet file path where this data was extracted from.
    pub file_path: String,
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

impl CueSheet {
    pub fn parse(file_path: String, input: &str) -> Self {
        let mut sheet = Self {
            file_path,
            ..Default::default()
        };
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
                        sheet.rem.push((key, value));
                    }
                }
                // TITLE can be part of the entire sheet or a TRACK.
                "TITLE" => {
                    if let Some(title) = extract_quoted_string(&line[5..]) {
                        if let Some(track) = &mut current_track {
                            track.title = Some(title);
                        } else if tab_count == 0 {
                            sheet.title = Some(title);
                        }
                    }
                }
                // PERFORMER can be part of the entire sheet or a TRACK.
                "PERFORMER" => {
                    if let Some(performer) = extract_quoted_string(&line[9..]) {
                        if let Some(track) = &mut current_track {
                            track.performer = Some(performer);
                        } else if tab_count == 0 {
                            sheet.performer = Some(performer);
                        }
                    }
                }
                // FILE can be part of the entire sheet or a TRACK.
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
                                sheet.file = Some(file);
                            }
                        }
                    }
                }
                "TRACK" => {
                    if let Some(track) = &mut current_track {
                        sheet.tracks.push(track.to_owned());
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
            sheet.tracks.push(track);
        }

        sheet
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
