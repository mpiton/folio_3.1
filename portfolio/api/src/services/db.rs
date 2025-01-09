use anyhow::Result;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type DbPool = Arc<Mutex<Connection>>;

pub async fn init_db() -> Result<DbPool> {
    let conn = Connection::open("portfolio.db")?;

    // Initialize tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contact_messages (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            subject TEXT NOT NULL,
            message TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS rss_feeds (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            link TEXT NOT NULL,
            description TEXT NOT NULL,
            published_at DATETIME NOT NULL
        )",
        [],
    )?;

    Ok(Arc::new(Mutex::new(conn)))
}
