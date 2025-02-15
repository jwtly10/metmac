#[derive(Debug)]
pub struct KeyEvent {
    pub key_name: String,
    pub window_title: String,
    pub timestamp: i64,
}

impl KeyEvent {
    pub fn new(key_name: String, window_title: String, timestamp: i64) -> KeyEvent {
        KeyEvent {
            key_name,
            window_title,
            timestamp,
        }
    }
}
