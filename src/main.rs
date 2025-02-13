mod input;
mod models;

use env_logger::init;
use rdev::{listen, Event, ListenError};

use crate::input::keyboard::handle_keyboard_event;

use log::info;

fn main() -> Result<(), ListenError> {
    init();
    info!("Starting MetMac...");
    listen(callback)?;
    Ok(())
}

fn callback(event: Event) {
    if handle_keyboard_event(&event).is_some() {
        // todo!()
    }
}
