use anyhow::{Context, Result};
use chrono::Utc;
use directories::BaseDirs;
use log::{debug, info};
use rusqlite::{params, Connection};
use std::fs;
use std::path::PathBuf;

use crate::models::events::KeyEvent;

#[derive()]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Self> {
        // Handling nice input strings like ~/.metmac etc
        let expanded_path = if path.starts_with("~") {
            let base_dirs = BaseDirs::new().context("Failed to get base directory")?;
            let home_dir = base_dirs.home_dir();

            let without_tilde = &path.strip_prefix("~").unwrap_or(&path);
            home_dir.join(without_tilde)
        } else {
            path.clone()
        };

        if let Some(parent) = expanded_path.parent() {
            if !parent.exists() {
                debug!("Creating database directory: {:?}", parent);
                fs::create_dir_all(parent)?;
            }
        }

        info!("Opening database connection at {:?}", expanded_path);
        let conn = Connection::open(expanded_path)?;
        Ok(Self { conn })
    }

    pub fn run_migrations(&mut self) -> Result<()> {
        info!("Running database migrations");

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at INTEGER NOT NULL
            )",
            [],
        )?;

        let mut migration_files: Vec<PathBuf> = fs::read_dir("src/storage/migrations")?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "sql"))
            .collect();

        migration_files.sort();

        info!("Found {} migration files to process", migration_files.len());

        let tx = self.conn.transaction()?;

        for path in migration_files {
            let filename = path.file_name().unwrap().to_string_lossy().into_owned();

            let count: i64 = tx.query_row(
                "SELECT COUNT(*) FROM migrations WHERE name = ?1",
                [&filename],
                |row| row.get(0),
            )?;

            if count == 0 {
                debug!("Applying migration: {}", filename);
                let sql = fs::read_to_string(&path)?;
                tx.execute_batch(&sql)?;

                tx.execute(
                    "INSERT INTO migrations (name, applied_at) VALUES (?1, ?2)",
                    (&filename, Utc::now().timestamp()),
                )?;

                info!("Migration {} applied successfully", filename);
            } else {
                debug!("Skipping migration: {}", filename);
            }
        }

        tx.commit()?;
        info!("Database migrations completed successfully!");

        Ok(())
    }

    pub fn insert_events(&mut self, events: &[KeyEvent]) -> Result<()> {
        debug!("Inserting {} events into the database", events.len());
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO events (event_timestamp, key_name, window_title) VALUES (?1, ?2, ?3)",
            )?;

            for event in events {
                debug!("Inserting event: {:?}", event);
                stmt.execute(params![event.timestamp, event.key_name, event.window_title,])?;
            }
        } // Scoped to drop borrow of tx

        tx.commit()?;
        Ok(())
    }

    pub fn get_events(&mut self) -> Result<Vec<KeyEvent>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, event_timestamp, key_name, window_title FROM events")?;
        let rows = stmt.query_map([], |row| {
            Ok(KeyEvent {
                timestamp: row.get(1)?,
                key_name: row.get(2)?,
                window_title: row.get(3)?,
            })
        })?;

        let mut events = Vec::new();
        for row in rows {
            events.push(row?);
        }

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_database_new() -> Result<()> {
        let tmp_dir = NamedTempFile::new()?;
        let tmp_db_path = PathBuf::from(tmp_dir.path());
        let mut db = Database::new(tmp_db_path.clone())?;
        db.run_migrations()?;

        // Test that the database file was created
        assert!(tmp_db_path.exists());

        Ok(())
    }

    #[test]
    fn test_database_insert_events() -> Result<()> {
        let tmp_dir = NamedTempFile::new()?;
        let tmp_db_path = PathBuf::from(tmp_dir.path());
        let mut db = Database::new(tmp_db_path.clone())?;
        db.run_migrations()?;

        let events = vec![
            KeyEvent {
                timestamp: Utc::now().timestamp(),
                key_name: "a".to_string(),
                window_title: "test".to_string(),
            },
            KeyEvent {
                timestamp: Utc::now().timestamp(),
                key_name: "b".to_string(),
                window_title: "test".to_string(),
            },
        ];

        match db.insert_events(&events) {
            Ok(_) => {}
            Err(e) => panic!("Failed to insert events: {:?}", e),
        }

        let db_events = match db.get_events() {
            Ok(events) => events,
            Err(e) => panic!("Failed to get events: {:?}", e),
        };
        assert_eq!(db_events.len(), 2);

        Ok(())
    }
}
