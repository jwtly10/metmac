use env_logger::init;
use rdev::{listen, Event, EventType, ListenError};

use log::{debug, info};

fn main() -> Result<(), ListenError> {
    init();

    info!("Starting MetMac...");

    listen(callback)?;
    Ok(())
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => {
            info!("Key press event: {:?}", key);
            debug!("Full event defaults: {:?}", event);
        }
        _ => {}
    }
}
