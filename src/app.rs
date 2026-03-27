use iced::{
    Alignment, Element, Task,
    widget::{button, column, row, text, text_input},
};

use crate::{bridge::BridgeHandle, model::TeamState};

#[derive(Default)]
pub struct Soulboard {
    pub state: TeamState,
    pub bridge: Option<BridgeHandle>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddA,
    SubtractA,
    AddB,
    SubtractB,
    ResetAll,
    SetDescription(String),
    SetNameA(String),
    SetNameB(String),
}

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
        Message::SetDescription(desc) => state.state.description = desc,
        Message::SetNameA(name) => state.state.team_a_name = name,
        Message::SetNameB(name) => state.state.team_b_name = name,
    }

    if let Some(bridge) = &state.bridge {
        bridge.publish_state(&state.state);
    }

    Task::none()
}

pub fn view(state: &Soulboard) -> Element<'_, Message> {
    let team_a_controls = row![
        button("-1").on_press(Message::SubtractA),
        button("+1").on_press(Message::AddA),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let team_b_controls = row![
        button("-1").on_press(Message::SubtractB),
        button("+1").on_press(Message::AddB),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    column![
        text("Soulboard").size(40),
        text("Native Iced app + external HTML bridge").size(18),
        text_input("Description", &state.state.description).on_input(Message::SetDescription),
        text_input("Team A name", &state.state.team_a_name).on_input(Message::SetNameA),
        text(state.state.team_a).size(32),
        team_a_controls,
        text_input("Team B name", &state.state.team_b_name).on_input(Message::SetNameB),
        text(state.state.team_b).size(32),
        team_b_controls,
        button("Reset All").on_press(Message::ResetAll),
        text("Bridge: ws://127.0.0.1:7878/ws").size(16),
    ]
    .spacing(16)
    .align_x(Alignment::Center)
    .into()
}
