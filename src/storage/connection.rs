use anyhow::{Context, Result};
use chrono::Utc;
use directories::BaseDirs;
use log::{debug, info};

use crate::models::events::KeyEvent;
use crate::models::stats::DashboardStats;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}
impl Database {
    pub async fn new(path: PathBuf) -> Result<Self> {
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
        let database_url = format!("sqlite:{}", expanded_path.to_string_lossy());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<()> {
        info!("Running database migrations");

        sqlx::migrate!("./migrations").run(&self.pool).await?;

        info!("Database migrations completed successfully!");
        Ok(())
    }

    pub async fn insert_events(&self, events: &[KeyEvent]) -> Result<()> {
        debug!("Inserting {} events into the database", events.len());

        let mut tx = self.pool.begin().await?;

        for event in events {
            debug!("Inserting event: {:?}", event);
            sqlx::query!(
                r#"
                INSERT INTO events (event_timestamp, key_name, window_title)
                VALUES (?, ?, ?)
                "#,
                event.timestamp,
                event.key_name,
                event.window_title
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_events(&self) -> Result<Vec<KeyEvent>> {
        debug!("Getting all events from the database");

        let events = sqlx::query_as!(
            KeyEvent,
            r#"
            SELECT
                event_timestamp as "timestamp",
                key_name,
                window_title
            FROM events
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(events)
    }

    pub async fn get_stats(&self) -> Result<DashboardStats> {
        debug!("Getting dashboard stats");

        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM events
            "#
        )
        .fetch_one(&self.pool)
        .await?
        .count;

        Ok(DashboardStats {
            total: total as i64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_database_new() -> Result<()> {
        let tmp_dir = NamedTempFile::new()?;
        let tmp_db_path = PathBuf::from(tmp_dir.path());
        let mut db = Database::new(tmp_db_path.clone()).await?;
        db.run_migrations().await?;

        // Test that the database file was created
        assert!(tmp_db_path.exists());

        Ok(())
    }

    #[tokio::test]
    async fn test_database_insert_events() -> Result<()> {
        let tmp_dir = NamedTempFile::new()?;
        let tmp_db_path = PathBuf::from(tmp_dir.path());
        let mut db = Database::new(tmp_db_path.clone()).await?;
        db.run_migrations().await?;

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

        db.insert_events(&events).await?;
        let db_events = db.get_events().await?;
        assert_eq!(db_events.len(), 2);

        Ok(())
    }
}
