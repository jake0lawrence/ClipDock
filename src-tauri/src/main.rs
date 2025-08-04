// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Connection;
use tokio::sync::broadcast;
use std::sync::Mutex;
use tauri::Manager;

mod db;
mod clipboard;
mod hotkeys;

struct AppState {
    conn: Mutex<Connection>,
    notifier: broadcast::Sender<()>,
}

fn main() {
    let conn = Connection::open("clipdock.db").unwrap();
    db::init(&conn).unwrap();

    let (tx, _rx) = broadcast::channel::<()>(8);
    let conn_for_thread = Connection::open("clipdock.db").unwrap();
    clipboard::spawn(conn_for_thread, tx.clone());

    tauri::Builder::default()
        .manage(AppState { conn: Mutex::new(conn), notifier: tx })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            hotkeys::register(&app.app_handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_clips, toggle_pin])
        .run(tauri::generate_context!())
        .expect("error while running app");
}

#[tauri::command]
fn get_clips(state: tauri::State<'_, AppState>) -> Vec<db::Clip> {
    let conn = state.conn.lock().unwrap();
    db::all(&conn).unwrap_or_default()
}

#[tauri::command]
fn toggle_pin(id: i64, state: tauri::State<'_, AppState>) {
    let conn = state.conn.lock().unwrap();
    let _ = db::toggle_pin(&conn, id);
}

