use std::{fs::File, io::BufReader};

use lofty::{
    file::{AudioFile, FileType, TaggedFileExt},
    probe::Probe,
    tag::{Accessor, ItemKey},
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum SongFromFileError {
    #[error("failed to parse song file: {0}")]
    Lofty(#[from] lofty::error::LoftyError),
    #[error("failed to read song file: {0}")]
    IO(#[from] std::io::Error),
    #[error("song has no metadata")]
    NoMetadata,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub file_path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub bpm: Option<f64>,
    pub duration_seconds: u64,
}

impl Song {
    /// Read a song's metadata from the given file path if it exists and is a supported format.
    pub fn from_file(path: &str) -> Result<Self, SongFromFileError> {
        let tagged_file = Self::try_open_file_probe(path)?.read()?;
        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => tagged_file
                .first_tag()
                .ok_or(SongFromFileError::NoMetadata)?,
        };

        let properties = tagged_file.properties();
        let duration = properties.duration();

        let bpm_string = tag
            .get_string(&ItemKey::Bpm)
            .or_else(|| tag.get_string(&ItemKey::IntegerBpm))
            .unwrap_or("");
        let bpm = match bpm_string.parse::<f64>() {
            Ok(bpm) => Some(bpm),
            Err(error) => {
                tracing::debug!(?error, "failed to parse bpm from string");
                None
            }
        };

        Ok(Self {
            file_path: path.to_string(),
            title: tag.title().map(String::from),
            artist: tag.artist().map(String::from),
            album: tag.album().map(String::from),
            genre: tag.genre().map(String::from),
            bpm,
            duration_seconds: duration.as_secs(),
        })
    }

    /// Trys to open probe expecting the format to match the file extension (`.flac`, `.mp3`, etc).
    /// If there is no extension or the extension isn't supported the `lofty` will try to guess the
    /// file type based on the file's contents.
    fn try_open_file_probe(path: &str) -> Result<Probe<BufReader<File>>, SongFromFileError> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);

        let Some(extension) = std::path::Path::new(path).extension() else {
            return Ok(Probe::new(reader).guess_file_type()?);
        };

        let Some(file_type) = FileType::from_ext(extension) else {
            return Ok(Probe::new(reader).guess_file_type()?);
        };

        Ok(Probe::new(reader).set_file_type(file_type))
    }
}
