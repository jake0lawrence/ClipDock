// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Connection;
use tokio::sync::broadcast;
use tauri::Manager;
use parking_lot::RwLock;
use std::fs;
use std::path::PathBuf;

mod db;
mod clipboard;
mod hotkeys;
mod autostart;

pub struct AppState {
    pub conn: Connection,
    pub notifier: broadcast::Sender<()>,
    pub settings: RwLock<Settings>,
    pub data_dir: PathBuf,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Settings {
    pub dark: bool,
    pub history: usize,
    pub autostart: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dark: false,
            history: 20,
            autostart: false,
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Get app data directory
            let data_dir = app.path().app_data_dir()
                .expect("failed to get app data directory");

            // Create directory if it doesn't exist
            fs::create_dir_all(&data_dir)
                .expect("failed to create data directory");

            // Database path
            let db_path = data_dir.join("clipdock.db");
            let conn = Connection::open(&db_path)
                .expect("failed to open database");
            db::init(&conn).expect("failed to initialize database");

            // Load settings from file
            let settings = load_settings_from_file(&data_dir);

            // Setup clipboard monitoring
            let (tx, _rx) = broadcast::channel::<()>(8);
            let conn_for_thread = Connection::open(&db_path)
                .expect("failed to open database for clipboard thread");
            clipboard::spawn(conn_for_thread, tx.clone());

            // Store state
            app.manage(AppState {
                conn,
                notifier: tx,
                settings: RwLock::new(settings),
                data_dir,
            });

            // Register hotkeys
            hotkeys::register(&app.app_handle());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_clips, toggle_pin, set_autostart, save_settings, load_settings, copy_to_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running app");
}

fn load_settings_from_file(data_dir: &PathBuf) -> Settings {
    let settings_path = data_dir.join("settings.json");
    if let Ok(contents) = fs::read_to_string(&settings_path) {
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        Settings::default()
    }
}

fn save_settings_to_file(data_dir: &PathBuf, settings: &Settings) -> Result<(), String> {
    let settings_path = data_dir.join("settings.json");
    let contents = serde_json::to_string_pretty(settings)
        .map_err(|e| e.to_string())?;
    fs::write(&settings_path, contents)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_clips(state: tauri::State<'_, AppState>) -> Vec<db::Clip> {
    db::all(&state.conn).unwrap_or_default()
}

#[tauri::command]
fn toggle_pin(id: i64, state: tauri::State<'_, AppState>) {
    let _ = db::toggle_pin(&state.conn, id);
}

#[tauri::command]
async fn set_autostart(enable: bool, state: tauri::State<'_, AppState>) -> Result<(), String> {
    autostart::apply(if enable { autostart::AutoStart::Enable } else { autostart::AutoStart::Disable })
        .map_err(|e| e.to_string())?;
    state.settings.write().autostart = enable;
    let settings = state.settings.read().clone();
    save_settings_to_file(&state.data_dir, &settings)?;
    Ok(())
}

#[tauri::command]
async fn save_settings(new: Settings, state: tauri::State<'_, AppState>) -> Result<(), String> {
    *state.settings.write() = new.clone();
    save_settings_to_file(&state.data_dir, &new)?;
    Ok(())
}

#[tauri::command]
async fn load_settings(state: tauri::State<'_, AppState>) -> Settings {
    state.settings.read().clone()
}

#[tauri::command]
async fn copy_to_clipboard(text: String) -> Result<(), String> {
    clipboard::write(&text)
}

