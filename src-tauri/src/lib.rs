mod cue;
mod fs_search;

#[tauri::command]
fn find_cue_sheets(dir: &str) -> Vec<cue::CueSheet> {
    let instant = std::time::Instant::now();
    let sheets = fs_search::find_cue_sheets(dir);
    let duration = instant.elapsed();
    tracing::info!(?duration, found=sheets.len(), "searched cue sheets");
    sheets
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![find_cue_sheets])
        .run(tauri::generate_context!())
        .unwrap_or_else(|error| {
            tracing::error!(?error, "error while running tauri application");
        });
}
