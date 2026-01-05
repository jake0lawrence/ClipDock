// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Connection;
use tokio::sync::broadcast;
use tauri::Manager;
use parking_lot::RwLock;

mod db;
mod clipboard;
mod hotkeys;
mod autostart;

pub struct AppState {
    pub conn: Connection,
    pub notifier: broadcast::Sender<()>,
    pub settings: RwLock<Settings>,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Settings {
    pub dark: bool,
    pub history: usize,
    pub autostart: bool,
}

fn main() {
    let conn = Connection::open("clipdock.db").unwrap();
    db::init(&conn).unwrap();

    let (tx, _rx) = broadcast::channel::<()>(8);
    let conn_for_thread = Connection::open("clipdock.db").unwrap();
    clipboard::spawn(conn_for_thread, tx.clone());

    tauri::Builder::default()
        .manage(AppState { conn, notifier: tx, settings: RwLock::new(Settings::default()) })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            hotkeys::register(&app.app_handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_clips, toggle_pin, set_autostart, save_settings, load_settings, copy_to_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running app");
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
    Ok(())
}

#[tauri::command]
async fn save_settings(new: Settings, state: tauri::State<'_, AppState>) {
    *state.settings.write() = new;
}

#[tauri::command]
async fn load_settings(state: tauri::State<'_, AppState>) -> Settings {
    state.settings.read().clone()
}

#[tauri::command]
async fn copy_to_clipboard(text: String) -> Result<(), String> {
    clipboard::write(&text)
}

