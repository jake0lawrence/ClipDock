use std::{sync::mpsc, thread, time::Duration};
use arboard::Clipboard;
use tokio::sync::broadcast;
use crate::db;

pub fn spawn(conn: rusqlite::Connection, notifier: broadcast::Sender<()>) {
    thread::spawn(move || {
        let mut cb = Clipboard::new().expect("clipboard");
        let mut last = String::new();

        loop {
            if let Ok(text) = cb.get_text() {
                if text != last {
                    last = text.clone();
                    let _ = db::push(&conn, &text);
                    let _ = notifier.send(());
                }
            }
            thread::sleep(Duration::from_millis(400));
        }
    });
}

pub fn write(text: &str) -> Result<(), String> {
    let mut cb = Clipboard::new().map_err(|e| e.to_string())?;
    cb.set_text(text).map_err(|e| e.to_string())?;
    Ok(())
}

