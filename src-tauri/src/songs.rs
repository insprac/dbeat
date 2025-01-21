use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
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
    file_path: String,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    genre: Option<String>,
    duration_seconds: u64,
}

impl Song {
    pub fn from_file(path: &str) -> Result<Self, SongFromFileError> {
        let tagged_file = Probe::open(path)?.read()?;
        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => tagged_file
                .first_tag()
                .ok_or(SongFromFileError::NoMetadata)?,
        };

        let properties = tagged_file.properties();
        let duration = properties.duration();

        Ok(Self {
            file_path: path.to_string(),
            title: tag.title().as_deref().map(String::from),
            artist: tag.artist().as_deref().map(String::from),
            album: tag.album().as_deref().map(String::from),
            genre: tag.genre().as_deref().map(String::from),
            duration_seconds: duration.as_secs(),
        })
    }
}
