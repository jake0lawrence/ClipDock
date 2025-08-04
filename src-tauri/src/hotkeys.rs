use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tauri::Manager;
use std::str::FromStr;

pub fn register(app: &tauri::AppHandle) {
    let handle = app.global_shortcut().clone();
    let shortcut = Shortcut::from_str("Ctrl+Shift+V").unwrap();
    handle
        .on_shortcut(shortcut, move |app, _, _| {
            if let Some(w) = app.get_webview_window("main") {
                if w.is_visible().unwrap_or(false) {
                    w.hide().ok();
                } else {
                    w.show().ok();
                    w.set_focus().ok();
                }
            }
        })
        .expect("register shortcut");

    let quick = Shortcut::from_str("Ctrl+Alt+V").unwrap();
    let handle = app.global_shortcut().clone();
    handle
        .on_shortcut(quick, move |app, _, _| {
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(clips) = app.invoke::<Vec<crate::db::Clip>>("get_clips").await {
                    if let Some(first) = clips.first() {
                        let _ = tauri::api::clipboard::write_text(app.app_handle(), first.text.clone());
                    }
                }
            });
        })
        .expect("register shortcut");
}

