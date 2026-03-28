//! Persistence helpers for saving and loading the application state.
//!
//! The functions here serialize and deserialize `TeamState` to a
//! JSON file on disk so the UI selections survive restarts.

use crate::model::TeamState;
use std::fs;
use std::io;

/// Filename used to persist the `TeamState`.
const STATE_FILE: &str = "soulboard_state.json";

/// Save the current `TeamState` to disk as pretty JSON.
pub fn save_state(state: &TeamState) -> io::Result<()> {
    let s = serde_json::to_string_pretty(state).map_err(io::Error::other)?;
    fs::write(STATE_FILE, s)
}

/// Load saved `TeamState` from disk, returning `None` if the file
/// is missing or cannot be parsed.
pub fn load_state() -> Option<TeamState> {
    match fs::read_to_string(STATE_FILE) {
        Ok(s) => match serde_json::from_str(&s) {
            Ok(st) => Some(st),
            Err(err) => {
                eprintln!("Failed to parse saved state: {}", err);
                None
            }
        },
        Err(_) => None,
    }
}
