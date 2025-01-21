use std::{borrow::Cow, process::Command, sync::Mutex};

use tauri::{Manager, State};

mod cue;
mod fs_search;
mod settings;

struct AppState<'a> {
    recordings_dir: Option<Cow<'a, str>>,
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        let recordings_dir = dirs::audio_dir().map(|mut dir| {
            dir.push("PioneerDJ/Recording");
            Cow::Owned(dir.to_string_lossy().into_owned())
        });
        Self { recordings_dir }
    }
}

#[tauri::command]
async fn get_default_path() -> Option<String> {
    let dir = AppState::default().recordings_dir;
    dir.map(|s| s.to_string())
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
    let state = state.lock().map_err(|err| {
        state.clear_poison();
        format!("app state lock was poisoned: {err}")
    })?;

    let dir = state
        .recordings_dir
        .clone()
        .ok_or_else(|| "recording directory not set".to_string())?;

    let instant = std::time::Instant::now();
    let sheets = fs_search::find_cue_sheets(&dir);
    let duration = instant.elapsed();
    tracing::info!(?duration, found = sheets.len(), "searched cue sheets");

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::default();

    tracing::info!(recordings_dir=?app_state.recordings_dir, "dbeat starting");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_default_path,
            get_cue_sheet,
            find_cue_sheets,
            open_file_location,
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|error| {
            tracing::error!(?error, "error while running tauri application");
        });
}
