//! UI layer: `Soulboard` application state, messages, and view/update logic.
//!
//! Public items in this module are the `Soulboard` struct, the `Message`
//! enum, and the `update` / `view` functions used by the `iced` application
//! runtime.

use crate::storage::save_state;
use crate::{
    bridge::BridgeHandle,
    model::{MapStatus, SlotWinner, TeamState},
};
use iced::{
    Alignment, Element, Task,
    widget::{button, column, combo_box, container, row},
};
use std::path::PathBuf;

/// Application state for the UI and bridge.
#[derive(Default)]
pub struct Soulboard {
    pub state: TeamState,
    pub bridge: Option<BridgeHandle>,
    pub available_teams: Vec<String>,
    pub team_a_combo_state: combo_box::State<String>,
    pub team_b_combo_state: combo_box::State<String>,
    pub slot_map_combo_states: Vec<combo_box::State<String>>,
    pub slot_mode_combo_states: Vec<combo_box::State<String>>,
    pub mode_line_map_combo_states: Vec<Vec<combo_box::State<String>>>,
    pub create_team_full_input: String,
    pub create_team_trunc_input: String,
    pub create_team_logo_path: Option<PathBuf>,
    pub create_team_feedback: String,
    pub create_team_feedback_is_error: bool,
    pub selected_tab: usize,
}

mod palette;
mod styles;
mod teams;

/// Messages/events handled by the UI.
#[derive(Debug, Clone)]
pub enum Message {
    AddA,
    SubtractA,
    AddB,
    SubtractB,
    SwapTeams,
    ResetAll,
    ClearPicksBans,
    SetDescription(String),
    SetCommentatorA(String),
    SetCommentatorB(String),

    SelectMap(usize, String),
    SelectMode(usize, String),
    CycleSlotWinner(usize),
    ToggleUse(usize, bool),
    SelectModeLineMap(usize, usize, String),
    ToggleModeLineStatus(usize, usize, MapStatus),
    SelectTeamA(String),
    SelectTeamB(String),
    SetCreateTeamFull(String),
    SetCreateTeamTrunc(String),
    PickCreateTeamLogo,
    CreateTeamLogoPicked(Option<PathBuf>),
    SubmitCreateTeam,
    SwitchTab(usize),
}

fn refresh_team_combo_states(state: &mut Soulboard) {
    let teams = teams::list_team_dirs();

    let team_a_selected = if state.state.team_a_dir.is_empty()
        || !teams.iter().any(|name| name == &state.state.team_a_dir)
    {
        None
    } else {
        Some(&state.state.team_a_dir)
    };
    let team_b_selected = if state.state.team_b_dir.is_empty()
        || !teams.iter().any(|name| name == &state.state.team_b_dir)
    {
        None
    } else {
        Some(&state.state.team_b_dir)
    };

    state.available_teams = teams.clone();
    state.team_a_combo_state = combo_box::State::with_selection(teams.clone(), team_a_selected);
    state.team_b_combo_state = combo_box::State::with_selection(teams, team_b_selected);
}

/// Apply a `Message` to the `Soulboard` state, publish it to the bridge,
/// and persist the updated state to disk.
pub fn update(state: &mut Soulboard, message: Message) -> Task<Message> {
    match message {
        Message::AddA => state.state.team_a += 1,
        Message::SubtractA => state.state.team_a -= 1,
        Message::AddB => state.state.team_b += 1,
        Message::SubtractB => state.state.team_b -= 1,
        Message::SwapTeams => {
            std::mem::swap(&mut state.state.team_a, &mut state.state.team_b);
            std::mem::swap(&mut state.state.team_a_name, &mut state.state.team_b_name);
            std::mem::swap(&mut state.state.team_a_full, &mut state.state.team_b_full);
            std::mem::swap(&mut state.state.team_a_trunc, &mut state.state.team_b_trunc);
            std::mem::swap(&mut state.state.team_a_dir, &mut state.state.team_b_dir);

            state.state.ensure_slot_winners_len();
            for winner in &mut state.state.slot_winners {
                *winner = match *winner {
                    SlotWinner::TeamA => SlotWinner::TeamB,
                    SlotWinner::TeamB => SlotWinner::TeamA,
                    SlotWinner::None => SlotWinner::None,
                };
            }
        }
        Message::ResetAll => {
            state.state.ensure_slot_winners_len();
            for winner in &mut state.state.slot_winners {
                *winner = SlotWinner::None;
            }

            state.state.team_a = 0;
            state.state.team_b = 0;
        }
        Message::ClearPicksBans => {
            for mode in &mut state.state.mode_lines {
                for entry in &mut mode.maps {
                    entry.status = MapStatus::None;
                }
            }
        }
        Message::SetDescription(desc) => state.state.description = desc,
        Message::SetCommentatorA(s) => state.state.commentator_a = s,
        Message::SetCommentatorB(s) => state.state.commentator_b = s,
        // cycle messages removed
        Message::SelectMap(idx, sel) => {
            if idx < state.state.map_mode_slots.len() {
                state.state.map_mode_slots[idx].0 = Some(sel.clone());
            }
        }
        Message::SelectMode(idx, sel) => {
            if idx < state.state.map_mode_slots.len() {
                state.state.map_mode_slots[idx].1 = Some(sel.clone());
            }
        }
        Message::CycleSlotWinner(idx) => {
            state.state.ensure_slot_winners_len();
            if idx < state.state.slot_winners.len() {
                state.state.slot_winners[idx] = match state.state.slot_winners[idx] {
                    SlotWinner::None => SlotWinner::TeamA,
                    SlotWinner::TeamA => SlotWinner::TeamB,
                    SlotWinner::TeamB => SlotWinner::None,
                };
            }
        }
        Message::SelectModeLineMap(mid, midx, sel) => {
            if mid < state.state.mode_lines.len() && midx < state.state.mode_lines[mid].maps.len() {
                state.state.mode_lines[mid].maps[midx].map = Some(sel.clone());
            }
        }
        Message::ToggleModeLineStatus(mid, midx, status) => {
            if mid < state.state.mode_lines.len() && midx < state.state.mode_lines[mid].maps.len() {
                let cur = state.state.mode_lines[mid].maps[midx].status.clone();
                if cur == status {
                    state.state.mode_lines[mid].maps[midx].status = MapStatus::None;
                } else {
                    state.state.mode_lines[mid].maps[midx].status = status;
                }
            }
        }
        Message::ToggleUse(idx, on) => {
            if on {
                state.state.selected_slot = Some(idx);
            } else if state.state.selected_slot == Some(idx) {
                state.state.selected_slot = None;
            }
        }
        Message::SelectTeamA(sel) => {
            if sel.is_empty() {
                state.state.team_a_full.clear();
                state.state.team_a_trunc.clear();
                state.state.team_a_name.clear();
                state.state.team_a_dir.clear();
            } else if let Some((full, trunc)) = teams::load_team_names(&sel) {
                state.state.team_a_full = full.clone();
                state.state.team_a_trunc = trunc.clone();
                state.state.team_a_name = trunc;
                state.state.team_a_dir = sel.clone();
            } else {
                state.state.team_a_full = sel.clone();
                state.state.team_a_trunc = sel.clone();
                state.state.team_a_name = sel.clone();
                state.state.team_a_dir = sel.clone();
            }
        }
        Message::SelectTeamB(sel) => {
            if sel.is_empty() {
                state.state.team_b_full.clear();
                state.state.team_b_trunc.clear();
                state.state.team_b_name.clear();
                state.state.team_b_dir.clear();
            } else if let Some((full, trunc)) = teams::load_team_names(&sel) {
                state.state.team_b_full = full.clone();
                state.state.team_b_trunc = trunc.clone();
                state.state.team_b_name = trunc;
                state.state.team_b_dir = sel.clone();
            } else {
                state.state.team_b_full = sel.clone();
                state.state.team_b_trunc = sel.clone();
                state.state.team_b_name = sel.clone();
                state.state.team_b_dir = sel.clone();
            }
        }
        Message::SetCreateTeamFull(value) => {
            state.create_team_full_input = value;
            state.create_team_feedback.clear();
            state.create_team_feedback_is_error = false;
        }
        Message::SetCreateTeamTrunc(value) => {
            state.create_team_trunc_input = value;
            state.create_team_feedback.clear();
            state.create_team_feedback_is_error = false;
        }
        Message::PickCreateTeamLogo => {
            return Task::perform(
                async { rfd::FileDialog::new().set_title("Select team logo").pick_file() },
                Message::CreateTeamLogoPicked,
            );
        }
        Message::CreateTeamLogoPicked(path) => {
            state.create_team_logo_path = path;
            state.create_team_feedback.clear();
            state.create_team_feedback_is_error = false;
        }
        Message::SubmitCreateTeam => {
            let full = state.create_team_full_input.trim().to_string();
            let trunc = state.create_team_trunc_input.trim().to_string();

            if full.is_empty() {
                state.create_team_feedback = "Full name is required".to_string();
                state.create_team_feedback_is_error = true;
            } else if trunc.is_empty() {
                state.create_team_feedback = "Short name is required".to_string();
                state.create_team_feedback_is_error = true;
            } else if state.create_team_logo_path.is_none() {
                state.create_team_feedback = "A logo file is required".to_string();
                state.create_team_feedback_is_error = true;
            } else {
                let logo = state.create_team_logo_path.as_deref();
                match teams::create_team(&full, &trunc, logo) {
                    Ok(()) => {
                        refresh_team_combo_states(state);
                        state.create_team_feedback =
                            format!("Team \"{}\" created successfully", full);
                        state.create_team_feedback_is_error = false;
                        state.create_team_full_input.clear();
                        state.create_team_trunc_input.clear();
                        state.create_team_logo_path = None;
                    }
                    Err(err) => {
                        state.create_team_feedback = err;
                        state.create_team_feedback_is_error = true;
                    }
                }
            }
        }
        Message::SwitchTab(idx) => {
            state.selected_tab = idx;
        }
    }

    // Team scores are derived from map winners.
    state.state.sync_scores_from_slot_winners();

    if let Some(bridge) = &state.bridge {
        bridge.publish_state(&state.state);
    }

    // persist the state to disk so selections survive app restarts
    if let Err(err) = save_state(&state.state) {
        eprintln!("Failed to save state: {}", err);
    }

    Task::none()
}
mod ui;

/// Build and return the main `iced` UI `Element` for the application.
pub fn view(state: &Soulboard) -> Element<'_, Message> {
    // Compose the main view from smaller, focused sections
    let left_container = container(ui::view_stream_and_teams(state))
        .width(iced::Length::FillPortion(3))
        .padding(10);

    // Tab buttons for the right panel
    let tabs = row![
        button("Map Slots")
            .on_press(Message::SwitchTab(0))
            .style(|_, status| button::Catalog::style(&styles::PrimaryButton, &(), status)),
        button("Pick / Ban")
            .on_press(Message::SwitchTab(1))
            .style(|_, status| button::Catalog::style(&styles::PrimaryButton, &(), status)),
        button("Create Team")
            .on_press(Message::SwitchTab(2))
            .style(|_, status| button::Catalog::style(&styles::PrimaryButton, &(), status)),
    ]
    .spacing(12);

    // Choose which right content to show based on current tab
    let right_content: Element<'_, Message> = match state.selected_tab {
        0 => ui::view_map_slots(state),
        1 => ui::view_mode_lines(state),
        2 => ui::view_team_creation(state),
        _ => ui::view_map_slots(state),
    };

    let right_scroll = iced::widget::scrollable(column![container(tabs).padding(6), right_content]);
    let right_container = container(right_scroll)
        .width(iced::Length::FillPortion(7))
        .padding(10);

    row![left_container, right_container]
        .spacing(40)
        .align_y(Alignment::Start)
        .into()
}
