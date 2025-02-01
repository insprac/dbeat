use rusqlite::Connection;

use crate::songs::Song;

/// All migrations are run when the database is initialised.
const MIGRATIONS: [&str; 1] = ["CREATE TABLE songs (
        file_path TEXT PRIMARY KEY,
        title TEXT,
        artist TEXT,
        album TEXT,
        genre TEXT,
        bpm FLOAT,
        duration_seconds INTEGER NOT NULL
    )"];

/// In memory database using sqlite, state is always wiped between session (no data is persisted).
/// All data is created from the user's files such as songs and recordings.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Creates a new in memory database and runs all migrations.
    pub fn init() -> rusqlite::Result<Self> {
        let conn = Connection::open_in_memory()?;

        for migration in MIGRATIONS {
            conn.execute(migration, ())?;
        }

        Ok(Self { conn })
    }

    pub fn insert_song(&self, song: &Song) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO songs (file_path, title, artist, album, genre, bpm, duration_seconds)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &song.file_path,
                &song.title,
                &song.artist,
                &song.album,
                &song.genre,
                &song.bpm,
                &song.duration_seconds,
            ),
        )?;

        Ok(())
    }

    pub fn list_songs(&self) -> rusqlite::Result<Vec<Song>> {
        let mut statement = self.conn.prepare(
            "SELECT file_path, title, artist, album, genre, bpm, duration_seconds FROM songs",
        )?;
        let mut rows = statement.query([])?;

        let mut songs = Vec::new();
        while let Some(row) = rows.next()? {
            songs.push(Song {
                file_path: row.get(0)?,
                title: row.get(1)?,
                artist: row.get(2)?,
                album: row.get(3)?,
                genre: row.get(4)?,
                bpm: row.get(5)?,
                duration_seconds: row.get(6)?,
            });
        }

        Ok(songs)
    }
}
