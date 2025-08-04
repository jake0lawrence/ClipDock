use rusqlite::{params, Connection, Result};

pub const MAX_HISTORY: usize = 20;

#[derive(serde::Serialize)]
pub struct Clip {
    pub id: i64,
    pub text: String,
    pub pinned: bool,
    pub ts: i64,
}

pub fn init(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS clips(
            id      INTEGER PRIMARY KEY,
            ts      INTEGER NOT NULL,
            text    TEXT    NOT NULL,
            pinned  INTEGER NOT NULL DEFAULT 0
        );",
    )
}

pub fn push(conn: &Connection, text: &str) -> Result<()> {
    if text.trim().is_empty() {
        return Ok(());
    }
    conn.execute(
        "INSERT INTO clips(ts, text) VALUES(strftime('%s','now'), ?1);",
        params![text],
    )?;
    conn.execute(
        &format!("DELETE FROM clips WHERE id NOT IN
                  (SELECT id FROM clips ORDER BY pinned DESC, ts DESC LIMIT {})",
                 MAX_HISTORY),
        [],
    )?;
    Ok(())
}

pub fn toggle_pin(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("UPDATE clips SET pinned = 1-pinned WHERE id=?1;", params![id])?;
    Ok(())
}

pub fn all(conn: &Connection) -> Result<Vec<Clip>> {
    let mut stmt = conn.prepare(
        "SELECT id, text, pinned, ts
         FROM clips
         ORDER BY pinned DESC, ts DESC;",
    )?;
    let rows = stmt
        .query_map([], |r| {
            Ok(Clip {
                id: r.get(0)?,
                text: r.get(1)?,
                pinned: r.get::<_, i64>(2)? == 1,
                ts: r.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

