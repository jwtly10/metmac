use metmac::input::keyboard::handle_keyboard_event;
use metmac::storage::{buffer::KeyEventBuffer, connection::Database};

use anyhow::Result;
use env_logger::init;

use log::info;
use rdev::{listen, Event};
use std::process::exit;
use std::sync::Arc;
use std::{path::PathBuf, time::Duration};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    init(); // Init env logger

    // TODO: Have some configuration logic, for path and flush
    let db_path = PathBuf::from("~/.metmac/data.db");
    let db = Database::new(db_path).await?;
    db.run_migrations().await?;

    let flush_threshold = 30; // events
    let flush_interval = 3; // seconds

    let buffer = KeyEventBuffer::new(db, flush_threshold, Duration::from_secs(flush_interval));

    let buffer_arc = Arc::new(Mutex::new(buffer));

    info!("Starting MetMac...");
    let listener_handle = tokio::task::spawn_blocking(move || {
        if let Err(e) = listen(move |event| {
            let buffer_arc_clone = buffer_arc.clone();
            tokio::spawn(async move {
                callback(event, buffer_arc_clone).await;
            });
        }) {
            info!("Error listening to events: {:?}", e);
        }
    });

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl-C, shutting down");
            exit(0);
        }
        _ = listener_handle => {
            info!("Listener task completed");
        }
    }

    info!("Exiting");
    Ok(())
}

async fn callback(event: Event, buffer_arc: Arc<Mutex<KeyEventBuffer>>) {
    if let Some(key_event) = handle_keyboard_event(&event) {
        let mut buffer = buffer_arc.lock().await;
        if let Err(e) = buffer.push(key_event).await {
            info!("Error pushing event to buffer: {:?}", e);
        }
    }
}
