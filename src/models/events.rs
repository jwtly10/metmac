#[derive(Debug)]
pub struct KeyEvent {
    pub key_name: String,
    pub timestamp: i64,
}

impl KeyEvent {
    pub fn new(key_name: String, timestamp: i64) -> KeyEvent {
        KeyEvent {
            key_name,
            timestamp,
        }
    }
}
