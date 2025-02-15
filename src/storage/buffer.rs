use anyhow::Result;
use log::{debug, warn};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::models::events::KeyEvent;

use super::connection::Database;

pub struct KeyEventBuffer {
    events: Vec<KeyEvent>,
    last_flush: Instant,
    flush_threshold: usize,
    flush_interval: Duration,

    db: Arc<Mutex<Database>>,
}

impl KeyEventBuffer {
    pub fn new(db: Arc<Mutex<Database>>, flush_threshold: usize, flush_interval: Duration) -> Self {
        Self {
            events: Vec::with_capacity(flush_threshold),
            last_flush: Instant::now(),
            flush_threshold,
            flush_interval,
            db,
        }
    }

    pub fn push(&mut self, key: KeyEvent) -> Result<()> {
        self.events.push(key);

        if self.should_flush() {
            debug!(
                "Buffer threshold reached, flushing {} events",
                self.events.len()
            );
            self.flush()?;
        }
        Ok(())
    }

    pub fn should_flush(&self) -> bool {
        self.events.len() >= self.flush_threshold
            || self.last_flush.elapsed() >= self.flush_interval
    }

    pub fn flush(&mut self) -> Result<()> {
        if self.events.is_empty() {
            return Ok(());
        }

        let batch = std::mem::take(&mut self.events);
        let batch_len = batch.len();

        match self.db.lock() {
            Ok(mut db) => {
                if let Err(e) = db.insert_events(&batch) {
                    warn!("failed to flush {} events to database: {}", batch_len, e);
                    self.events = batch;
                    return Err(e);
                }

                self.last_flush = Instant::now();
                Ok(())
            }
            Err(e) => {
                self.events = batch;
                Err(anyhow::anyhow!("Failed to lock database mutex: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use chrono::Utc;
    use tempfile::NamedTempFile;

    #[test]
    fn test_buffer_flush_by_threshold() -> Result<()> {
        // Setup database
        let tmp_db = NamedTempFile::new()?;
        let tmp_db_path = PathBuf::from(tmp_db.path());
        let mut db = Database::new(tmp_db_path)?;
        db.run_migrations()?;
        let db_arc = Arc::new(Mutex::new(db));

        let flush_threshold = 5;
        let flush_interval = 60; // Long enough to not trigger by time

        let mut buffer =
            KeyEventBuffer::new(db_arc, flush_threshold, Duration::from_secs(flush_interval));

        for i in 0..5 {
            match buffer.push(KeyEvent::new(
                format!("key{}", i),
                format!("title{}", i),
                Utc::now().timestamp(),
            )) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Failed to push event to buffer: {}", e);
                }
            }
        } // This should trigger flush by threshold

        let mut db = buffer.db.lock().unwrap();
        let events = match db.get_events() {
            Ok(events) => events,
            Err(e) => {
                panic!("Failed to get events from database: {}", e);
            }
        };

        assert_eq!(events.len(), 5);

        Ok(())
    }

    #[test]
    fn test_buffer_flush_by_interval() -> Result<()> {
        // Setup database
        let tmp_db = NamedTempFile::new()?;
        let tmp_db_path = PathBuf::from(tmp_db.path());
        let mut db = Database::new(tmp_db_path)?;
        db.run_migrations()?;
        let db_arc = Arc::new(Mutex::new(db));

        let flush_threshold = 10; // Large enough to not trigger by threshold

        // Note: This is assuming pushing to buffer is essentially instant
        // this ensures the unit test runs quickly but *may* cause flakiness
        let flush_interval = 1;

        let mut buffer =
            KeyEventBuffer::new(db_arc, flush_threshold, Duration::from_secs(flush_interval));

        for i in 0..5 {
            match buffer.push(KeyEvent::new(
                format!("key{}", i),
                format!("title{}", i),
                Utc::now().timestamp(),
            )) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Failed to push event to buffer: {}", e);
                }
            }
        }

        // Sleep and then trigger a new keypress to trigger flush by interval
        std::thread::sleep(Duration::from_secs(flush_interval + 1));

        match buffer.push(KeyEvent::new(
            "key6".to_string(),
            "title6".to_string(),
            Utc::now().timestamp(),
        )) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to push event to buffer: {}", e);
            }
        }

        let mut db = buffer.db.lock().unwrap();
        let events = match db.get_events() {
            Ok(events) => events,
            Err(e) => {
                panic!("Failed to get events from database: {}", e);
            }
        };

        assert_eq!(events.len(), 6);

        Ok(())
    }
}
