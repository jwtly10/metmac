use anyhow::Result;
use log::{debug, warn};
use std::time::{Duration, Instant};

use crate::models::events::KeyEvent;

use super::connection::Database;

pub struct KeyEventBuffer {
    events: Vec<KeyEvent>,
    last_flush: Instant,
    flush_threshold: usize,
    flush_interval: Duration,

    db: Database,
}

impl KeyEventBuffer {
    pub fn new(db: Database, flush_threshold: usize, flush_interval: Duration) -> Self {
        Self {
            events: Vec::with_capacity(flush_threshold),
            last_flush: Instant::now(),
            flush_threshold,
            flush_interval,
            db,
        }
    }

    pub async fn push(&mut self, key: KeyEvent) -> Result<()> {
        self.events.push(key);

        if self.should_flush() {
            debug!(
                "Buffer threshold reached, flushing {} events",
                self.events.len()
            );
            self.flush().await?;
        }
        Ok(())
    }

    pub fn should_flush(&self) -> bool {
        self.events.len() >= self.flush_threshold
            || self.last_flush.elapsed() >= self.flush_interval
    }

    pub async fn flush(&mut self) -> Result<()> {
        if self.events.is_empty() {
            return Ok(());
        }

        let batch = std::mem::take(&mut self.events);
        let batch_len = batch.len();

        if let Err(e) = self.db.insert_events(&batch).await {
            warn!("failed to flush {} events to database: {}", batch_len, e);
            self.events = batch;
            return Err(e);
        }

        self.last_flush = Instant::now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use chrono::Utc;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_buffer_flush_by_threshold() -> Result<()> {
        // Setup database
        let tmp_db = NamedTempFile::new()?;
        let tmp_db_path = PathBuf::from(tmp_db.path());
        let db = Database::new(tmp_db_path).await?;
        db.run_migrations().await?;

        let flush_threshold = 5;
        let flush_interval = 60; // Long enough to not trigger by time

        let mut buffer =
            KeyEventBuffer::new(db, flush_threshold, Duration::from_secs(flush_interval));

        for i in 0..5 {
            buffer
                .push(KeyEvent {
                    timestamp: Utc::now().timestamp(),
                    key_name: format!("key{}", i),
                })
                .await?;
        }

        let events = buffer.db.get_events().await?;
        assert_eq!(events.len(), 5);

        Ok(())
    }

    #[tokio::test]
    async fn test_buffer_flush_by_interval() -> Result<()> {
        // Setup database
        let tmp_db = NamedTempFile::new()?;
        let tmp_db_path = PathBuf::from(tmp_db.path());
        let db = Database::new(tmp_db_path).await?;
        db.run_migrations().await?;

        let flush_threshold = 10; // Large enough to not trigger by threshold

        // Note: This is assuming pushing to buffer is essentially instant
        // this ensures the unit test runs quickly but *may* cause flakiness
        let flush_interval = 1;

        let mut buffer =
            KeyEventBuffer::new(db, flush_threshold, Duration::from_secs(flush_interval));

        for i in 0..5 {
            buffer
                .push(KeyEvent {
                    timestamp: Utc::now().timestamp(),
                    key_name: format!("key{}", i),
                })
                .await?;
        }

        // Sleep and then trigger a new keypress to trigger flush by interval
        tokio::time::sleep(Duration::from_secs(flush_interval + 1)).await;

        buffer
            .push(KeyEvent::new("key6".to_string(), Utc::now().timestamp()))
            .await?;

        let events = buffer.db.get_events().await?;
        assert_eq!(events.len(), 6);

        Ok(())
    }
}
