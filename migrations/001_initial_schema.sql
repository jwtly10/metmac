-- 001_initial_schema.sql
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_timestamp INTEGER NOT NULL,
    key_name TEXT NOT NULL,
    window_title TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_events_timestamps
ON events(event_timestamp);
