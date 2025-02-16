-- due to complications with using mac osx and the window title
-- along with our keypress tracking, removing window title functionality for now
CREATE TABLE IF NOT EXISTS events_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_timestamp INTEGER NOT NULL,
    key_name TEXT NOT NULL
);

INSERT INTO events_new (id, event_timestamp, key_name)
SELECT id, event_timestamp, key_name
FROM events;

DROP VIEW IF EXISTS today_events;

DROP TABLE events;

ALTER TABLE events_new RENAME TO events;

CREATE INDEX IF NOT EXISTS idx_events_timestamps
ON events(event_timestamp);

CREATE VIEW IF NOT EXISTS today_events AS
SELECT id, event_timestamp, key_name FROM events
WHERE date(event_timestamp / 1000, 'unixepoch') = date('now', 'utc');
