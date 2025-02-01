use std::{borrow::Cow, process::Command, sync::Mutex};

use db::Database;
use tauri::{Manager, State};

mod cue;
mod db;
mod fs_search;
mod songs;

/// The core app state handled by Tauri and passed into commands, etc.
#[derive(Debug)]
struct AppState<'a> {
    music_dir: Option<Cow<'a, str>>,
    recordings_dir: Option<Cow<'a, str>>,
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        let music_dir = dirs::audio_dir().map(|dir| Cow::Owned(dir.to_string_lossy().into_owned()));
        let recordings_dir = dirs::audio_dir().map(|mut dir| {
            dir.push("PioneerDJ/Recording");
            Cow::Owned(dir.to_string_lossy().into_owned())
        });
        Self {
            music_dir,
            recordings_dir,
        }
    }
}

#[tauri::command]
async fn get_recordings_dir(
    state: State<'_, Mutex<AppState<'_>>>,
) -> Result<Option<String>, String> {
    let state = state.lock().unwrap();
    Ok(state.recordings_dir.clone().map(|s| s.to_string()))
}

#[tauri::command]
async fn get_music_dir(state: State<'_, Mutex<AppState<'_>>>) -> Result<Option<String>, String> {
    let state = state.lock().unwrap();
    Ok(state.music_dir.clone().map(|s| s.to_string()))
}

#[tauri::command]
async fn get_cue_sheet(path: &str) -> Result<cue::CueSheet, String> {
    fs_search::read_cue_sheet(path).map_err(|error| {
        tracing::error!(?error, "failed to read file");
        error.to_string()
    })
}

#[tauri::command]
async fn find_cue_sheets(
    state: State<'_, Mutex<AppState<'_>>>,
) -> Result<Vec<cue::CueSheet>, String> {
    let state = state
        .lock()
        .map_err(|error| format!("app state lock was poisoned: {error}"))?;

    let dir = state
        .recordings_dir
        .clone()
        .ok_or_else(|| "recording directory not set".to_string())?;

    let instant = std::time::Instant::now();
    let sheets = fs_search::find_cue_sheets(&dir);
    let duration = instant.elapsed();
    tracing::debug!(?duration, found = sheets.len(), "searched cue sheets");

    Ok(sheets)
}

#[tauri::command]
async fn open_file_location(path: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
async fn get_song(database: State<'_, Mutex<Database>>, path: &str) -> Result<songs::Song, String> {
    let database = database.lock().unwrap();
    database.get_song(path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn find_songs(database: State<'_, Mutex<Database>>) -> Result<Vec<songs::Song>, String> {
    let database = database.lock().unwrap();
    database.list_songs().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::default();

    tracing::info!(state=?app_state, "dbeat starting");

    let database = Database::init().unwrap();

    // Seed the database with songs from the music dir.
    match app_state.music_dir.clone().map(|dir| dir.to_string()) {
        Some(music_dir) => {
            for song in fs_search::find_songs(&music_dir) {
                if let Err(error) = database.insert_song(&song) {
                    tracing::error!(?error, "failed to insert song");
                }
            }
        }
        None => {
            tracing::warn!("music dir not set, can't load songs into the database");
        }
    };

    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            app.manage(Mutex::new(database));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_recordings_dir,
            get_music_dir,
            get_cue_sheet,
            find_cue_sheets,
            open_file_location,
            find_songs,
            get_song,
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|error| {
            tracing::error!(?error, "error while running tauri application");
        });
}
