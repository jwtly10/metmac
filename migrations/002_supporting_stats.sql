CREATE VIEW IF NOT EXISTS today_events AS
SELECT * FROM events
WHERE date (event_timestamp / 1000, 'unixepoch') = date ('now', 'utc');
