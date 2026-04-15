//! Small overlay manager for Splatoon competitive broadcasting
//!
//! This crate contains the binary application entrypoint. Run `cargo run`
//! to start the application. Generate API docs with `cargo doc --no-deps`.
//!
//! The project uses `rustdoc` for API documentation and `mdBook` for user
//! guides. Convert inline `//` comments to `///` (or `//!` for module-level)
//! comments so they appear in generated documentation.

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use iced::Task;

mod app;
mod bridge;
mod model;
mod storage;

use crate::storage::load_state;
use app::{Soulboard, update, view};
use bridge::start_bridge;
use iced::widget::combo_box;

fn main() -> iced::Result {
    iced::application("Soulboard", update, view)
        .centered()
        .run_with(|| {
            let state = load_state().unwrap_or_default();

            // load available maps and modes from assets directory
            let mut maps = Vec::new();
            // include an explicit blank option as the first choice
            maps.push(String::new());
            if let Ok(entries) = std::fs::read_dir("assets/maps") {
                for e in entries.flatten() {
                    if let Some(stem) = e.path().file_stem() {
                        maps.push(stem.to_string_lossy().to_string());
                    }
                }
            }

            let mut modes = Vec::new();
            // include an explicit blank option as the first choice
            modes.push(String::new());
            if let Ok(entries) = std::fs::read_dir("assets/modes") {
                for e in entries.flatten() {
                    if let Some(stem) = e.path().file_stem() {
                        modes.push(stem.to_string_lossy().to_string());
                    }
                }
            }

            let mut teams = Vec::new();
            // include an explicit blank option as the first choice
            teams.push(String::new());
            if let Ok(entries) = std::fs::read_dir("assets/teams") {
                for e in entries.flatten() {
                    if e.path().is_dir()
                        && let Some(name) = e.file_name().to_str()
                    {
                        teams.push(name.to_string());
                    }
                }
            }

            let team_a_combo_state = combo_box::State::with_selection(
                teams.clone(),
                if state.team_a_dir.is_empty() {
                    None
                } else {
                    Some(&state.team_a_dir)
                },
            );
            let team_b_combo_state = combo_box::State::with_selection(
                teams.clone(),
                if state.team_b_dir.is_empty() {
                    None
                } else {
                    Some(&state.team_b_dir)
                },
            );

            let slot_map_combo_states = state
                .map_mode_slots
                .iter()
                .map(|(map, _)| combo_box::State::with_selection(maps.clone(), map.as_ref()))
                .collect();
            let slot_mode_combo_states = state
                .map_mode_slots
                .iter()
                .map(|(_, mode)| combo_box::State::with_selection(modes.clone(), mode.as_ref()))
                .collect();
            let mode_line_map_combo_states = state
                .mode_lines
                .iter()
                .map(|line| {
                    line.maps
                        .iter()
                        .map(|entry| combo_box::State::with_selection(maps.clone(), entry.map.as_ref()))
                        .collect()
                })
                .collect();

            (
                Soulboard {
                    state: state.clone(),
                    bridge: Some(start_bridge(state)),
                    team_a_combo_state,
                    team_b_combo_state,
                    slot_map_combo_states,
                    slot_mode_combo_states,
                    mode_line_map_combo_states,
                    selected_tab: 0,
                },
                Task::none(),
            )
        })
}
