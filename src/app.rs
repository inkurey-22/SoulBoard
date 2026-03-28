//! UI layer: `Soulboard` application state, messages, and view/update logic.
//!
//! Public items in this module are the `Soulboard` struct, the `Message`
//! enum, and the `update` / `view` functions used by the `iced` application
//! runtime.

use crate::storage::save_state;
use crate::{
    bridge::BridgeHandle,
    model::{MapStatus, TeamState},
};
use iced::{
    Alignment, Element, Task,
    widget::{button, column, container, row},
};

/// Application state for the UI and bridge.
#[derive(Default)]
pub struct Soulboard {
    pub state: TeamState,
    pub bridge: Option<BridgeHandle>,
    pub available_maps: Vec<String>,
    pub available_modes: Vec<String>,
    pub available_teams: Vec<String>,
    pub selected_tab: usize,
}

mod teams;
mod palette;
mod styles;

/// Messages/events handled by the UI.
#[derive(Debug, Clone)]
pub enum Message {
    AddA,
    SubtractA,
    AddB,
    SubtractB,
    ResetAll,
    ClearPicksBans,
    SetDescription(String),
    SetCommentatorA(String),
    SetCommentatorB(String),

    SelectMap(usize, String),
    SelectMode(usize, String),
    ToggleUse(usize, bool),
    SelectModeLineMap(usize, usize, String),
    ToggleModeLineStatus(usize, usize, MapStatus),
    SelectTeamA(String),
    SelectTeamB(String),
    SwitchTab(usize),
}

/// Apply a `Message` to the `Soulboard` state, publish it to the bridge,
/// and persist the updated state to disk.
pub fn update(state: &mut Soulboard, message: Message) -> Task<Message> {
    match message {
        Message::AddA => state.state.team_a += 1,
        Message::SubtractA => state.state.team_a -= 1,
        Message::AddB => state.state.team_b += 1,
        Message::SubtractB => state.state.team_b -= 1,
        Message::ResetAll => {
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
        Message::SwitchTab(idx) => {
            state.selected_tab = idx;
        }
    }

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
    ]
    .spacing(12);

    // Choose which right content to show based on current tab
    let right_content: Element<'_, Message> = if state.selected_tab == 0 {
        ui::view_map_slots(state)
    } else {
        ui::view_mode_lines(state)
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
