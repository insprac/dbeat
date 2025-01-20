use tauri_plugin_store::Store;

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub recordings_dir: Option<String>,
}

impl AppSettings {
    pub fn load_from_store<R: tauri::Runtime>(
        store: &Store<R>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let recordings_dir = store
            .get("settings.recordings_dir")
            .and_then(|v| v.as_str().map(|s| Some(s.to_string())))
            .unwrap_or_else(Self::default_recordings_dir);

        Ok(Self { recordings_dir })
    }

    fn default_recordings_dir() -> Option<String> {
        let Some(mut dir) = dirs::audio_dir() else {
            return None;
        };
        dir.push("PioneerDJ/Recording");
        dir.to_str().map(|s| s.to_string())
    }
}
