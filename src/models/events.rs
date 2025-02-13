#[derive(Debug)]
pub struct KeyEvent {
    key_name: String,
    application: String,
    timestamp: i64,
}

impl KeyEvent {
    pub fn new(key_name: String, application: String, timestamp: i64) -> KeyEvent {
        KeyEvent {
            key_name,
            application,
            timestamp,
        }
    }
}
