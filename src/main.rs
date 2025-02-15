mod input;
mod models;
mod storage;

use anyhow::{Error, Result};

use env_logger::init;
use rdev::{listen, Event};
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex},
    time::Duration,
};
use storage::{buffer::KeyEventBuffer, connection::Database};

use crate::input::keyboard::handle_keyboard_event;

use log::info;

fn main() -> Result<()> {
    init(); // Init env logger

    // TODO: Have some configuration logic, for path and flush
    let db_path = PathBuf::from("~/.metmac/data.db");
    let mut db = Database::new(db_path)?;
    db.run_migrations()?;

    let flush_threshold = 30; // events
    let flush_interval = 3; // seconds

    let db_arc = Arc::new(Mutex::new(db));

    let buffer = KeyEventBuffer::new(db_arc, flush_threshold, Duration::from_secs(flush_interval));

    let buffer_rc = Rc::new(RefCell::new(buffer));

    info!("Starting MetMac...");
    if let Err(e) = listen({
        let buffer_rc_clone = buffer_rc.clone();
        move |event| {
            callback(event, buffer_rc_clone.clone());
        }
    }) {
        return Err(Error::msg(format!("{:?}", e)));
    }

    Ok(())
}

fn callback(event: Event, buffer_rc: Rc<RefCell<KeyEventBuffer>>) {
    let mut buffer = buffer_rc.borrow_mut();
    if let Some(key_event) = handle_keyboard_event(&event) {
        match buffer.push(key_event) {
            Ok(_) => (),
            Err(e) => {
                info!("Error pushing event to buffer: {:?}", e);
            }
        }
    }
}
