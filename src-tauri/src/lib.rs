mod fs_explorer;
mod launcher;
mod library;
mod steam;

use library::{CustomGame, Library};
use steam::SteamGame;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

// ---------------------------------------------------------------------------
// Shared state
// ---------------------------------------------------------------------------

struct AppState {
    library: Mutex<Library>,
}

fn library_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("could not resolve app data dir")
        .join("custom_games.json")
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

#[tauri::command]
fn get_steam_games() -> Result<Vec<SteamGame>, String> {
    steam::discover_games().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_custom_games(state: State<AppState>) -> Vec<CustomGame> {
    state.library.lock().unwrap().games().to_vec()
}

#[tauri::command]
fn add_game(
    state: State<AppState>,
    title: String,
    executable: String,
    cover_image: Option<String>,
    tags: Vec<String>,
    notes: Option<String>,
) -> Result<CustomGame, String> {
    let game = CustomGame::new(
        title,
        executable,
        cover_image.map(PathBuf::from),
        tags,
        notes,
    );
    state
        .library
        .lock()
        .unwrap()
        .add(game)
        .map(|g| g.clone())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_game(state: State<AppState>, id: String) -> Result<(), String> {
    state
        .library
        .lock()
        .unwrap()
        .remove(&id)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn launch_game(
    _state: State<AppState>,
    _key: String,
    app_id: Option<u32>,
    executable: Option<String>,
) -> Result<(), String> {
    match (app_id, executable) {
        (Some(id), _) => launcher::launch_steam(id).map_err(|e| e.to_string()),
        (None, Some(path)) => {
            launcher::spawn_executable(&path).map_err(|e| e.to_string())?;
            Ok(())
        }
        (None, None) => Err("Either app_id or executable must be provided".to_string()),
    }
}

// ---------------------------------------------------------------------------
// File-explorer commands
// ---------------------------------------------------------------------------

#[tauri::command]
fn list_directory(path: String) -> Result<Vec<fs_explorer::DirEntry>, String> {
    fs_explorer::read_dir(&path)
}

#[tauri::command]
fn get_file_explorer_bookmarks() -> Vec<fs_explorer::Bookmark> {
    fs_explorer::get_bookmarks()
}


// ---------------------------------------------------------------------------
// App entry point
// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let path = library_path(app.handle());
            let library = Library::load(path).expect("failed to load game library");
            app.manage(AppState {
                library: Mutex::new(library),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_steam_games,
            get_custom_games,
            add_game,
            remove_game,
            launch_game,
            list_directory,
            get_file_explorer_bookmarks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
