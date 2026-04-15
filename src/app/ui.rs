use super::{Message, Soulboard};
use crate::app::palette as pal;
use crate::app::styles;
use crate::model::MapStatus;
use iced::{
    Alignment, Element,
    widget::{button, checkbox, column, combo_box, container, row, text, text_input},
};

pub(super) fn view_stream_and_teams(state: &Soulboard) -> Element<'_, Message> {
    let team_a_selected = if state.state.team_a_dir.is_empty() {
        None
    } else {
        Some(&state.state.team_a_dir)
    };
    let team_b_selected = if state.state.team_b_dir.is_empty() {
        None
    } else {
        Some(&state.state.team_b_dir)
    };
    let team_a_pick = combo_box(
        &state.team_a_combo_state,
        "Select team A",
        team_a_selected,
        Message::SelectTeamA,
    )
    .input_style(styles::input_style)
    .menu_style(styles::dropdown_menu_style);
    let team_b_pick = combo_box(
        &state.team_b_combo_state,
        "Select team B",
        team_b_selected,
        Message::SelectTeamB,
    )
    .input_style(styles::input_style)
    .menu_style(styles::dropdown_menu_style);

    let team_a_controls = row![
        button("-1")
            .on_press(Message::SubtractA)
            .style(styles::primary_button_style),
        button("+1")
            .on_press(Message::AddA)
            .style(styles::primary_button_style),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let team_b_controls = row![
        button("-1")
            .on_press(Message::SubtractB)
            .style(styles::primary_button_style),
        button("+1")
            .on_press(Message::AddB)
            .style(styles::primary_button_style),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let left = column![
        text("Soulboard").size(42).color(pal::red()),
        container(text("Stream Info").size(16).color(pal::subtext1())).padding(8),
        text_input("Description", &state.state.description)
            .on_input(Message::SetDescription)
            .style(styles::input_style),
        text_input("Commentator A", &state.state.commentator_a)
            .on_input(Message::SetCommentatorA)
            .style(styles::input_style),
        text_input("Commentator B", &state.state.commentator_b)
            .on_input(Message::SetCommentatorB)
            .style(styles::input_style),
        container(text("Teams").size(16).color(pal::subtext1())).padding(8),
        container(team_a_pick).width(iced::Length::Fill),
        text(state.state.team_a).size(32).color(pal::text()),
        team_a_controls,
        container(team_b_pick).width(iced::Length::Fill),
        text(state.state.team_b).size(32).color(pal::text()),
        team_b_controls,
        button("Reset Score")
            .on_press(Message::ResetAll)
            .style(styles::primary_button_style),
        button("Clear Picks/Bans")
            .on_press(Message::ClearPicksBans)
            .style(styles::primary_button_style),
        text("Bridge: ws://127.0.0.1:7878/ws")
            .size(14)
            .color(pal::subtext0()),
    ]
    .spacing(14)
    .align_x(Alignment::Center);

    container(left)
        .style(|_| container::Catalog::style(&styles::Card, &()))
        .into()
}

pub(super) fn view_map_slots(state: &Soulboard) -> Element<'_, Message> {
    let mut map_rows = column![];

    // Header for map slots
    map_rows = map_rows.push(container(text("Map Slots").color(pal::red()).size(20)).padding(6));

    // header row with explicit columns
    map_rows = map_rows.push(
        row![
            container(text("Use").color(pal::subtext1())).width(iced::Length::FillPortion(1)),
            container(text("Map").color(pal::subtext1())).width(iced::Length::FillPortion(4)),
            container(text("Mode").color(pal::subtext1())).width(iced::Length::FillPortion(4)),
        ]
        .spacing(20),
    );

    for i in 0..9 {
        let slot = state.state.map_mode_slots.get(i);
        let map_selected = slot.and_then(|(map, _)| map.as_ref());
        let mode_selected = slot.and_then(|(_, mode)| mode.as_ref());

        let map_combo_state = match state.slot_map_combo_states.get(i) {
            Some(combo_state) => combo_state,
            None => continue,
        };
        let mode_combo_state = match state.slot_mode_combo_states.get(i) {
            Some(combo_state) => combo_state,
            None => continue,
        };

        let map_pick = combo_box(map_combo_state, "Select map", map_selected, move |s| {
            Message::SelectMap(i, s)
        })
        .input_style(styles::input_style)
        .menu_style(styles::dropdown_menu_style);
        let mode_pick = combo_box(mode_combo_state, "Select mode", mode_selected, move |s| {
            Message::SelectMode(i, s)
        })
        .input_style(styles::input_style)
        .menu_style(styles::dropdown_menu_style);

        let is_selected = state.state.selected_slot == Some(i);
        let use_checkbox = checkbox("", is_selected)
            .on_toggle(move |b| Message::ToggleUse(i, b))
            .style(styles::checkbox_style);

        let row_elem = row![
            container(use_checkbox).width(iced::Length::FillPortion(1)),
            container(map_pick).width(iced::Length::FillPortion(4)),
            container(mode_pick).width(iced::Length::FillPortion(4)),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        map_rows = map_rows.push(row_elem).spacing(6);
    }

    container(map_rows)
        .padding(8)
        .style(|_| container::Catalog::style(&styles::Card, &()))
        .into()
}

pub(super) fn view_mode_lines(state: &Soulboard) -> Element<'_, Message> {
    // Render modes as columns, each column contains the lines of maps for that mode
    let mut modes_row = row![];

    for (mi, mode) in state.state.mode_lines.iter().enumerate() {
        let mut col = column![];

        // mode name at top of column
        col = col.push(container(text(&mode.name).color(pal::red()).size(18)).padding(6));

        for (mj, _entry) in mode.maps.iter().enumerate() {
            let current = state.state.mode_lines[mi].maps[mj].map.as_ref();
            let map_combo_state = match state
                .mode_line_map_combo_states
                .get(mi)
                .and_then(|row| row.get(mj))
            {
                Some(combo_state) => combo_state,
                None => continue,
            };

            let map_pick = combo_box(map_combo_state, "Select map", current, move |s| {
                Message::SelectModeLineMap(mi, mj, s)
            })
            .input_style(styles::input_style)
            .menu_style(styles::dropdown_menu_style);

            let is_banned = state.state.mode_lines[mi].maps[mj].status == MapStatus::Banned;
            let is_picked = state.state.mode_lines[mi].maps[mj].status == MapStatus::Picked;

            let banned_chk = checkbox("B", is_banned)
                .on_toggle(move |b| {
                    if b {
                        Message::ToggleModeLineStatus(mi, mj, MapStatus::Banned)
                    } else {
                        Message::ToggleModeLineStatus(mi, mj, MapStatus::None)
                    }
                })
                .style(styles::checkbox_style);

            let picked_chk = checkbox("P", is_picked)
                .on_toggle(move |b| {
                    if b {
                        Message::ToggleModeLineStatus(mi, mj, MapStatus::Picked)
                    } else {
                        Message::ToggleModeLineStatus(mi, mj, MapStatus::None)
                    }
                })
                .style(styles::checkbox_style);

            let cell = column![map_pick, row![banned_chk, picked_chk].spacing(6)]
                .spacing(6)
                .width(iced::Length::FillPortion(1));

            col = col.push(cell);
        }

        modes_row = modes_row.push(
            container(col)
                .padding(6)
                .width(iced::Length::FillPortion(1))
                .style(styles::card_style),
        );
    }

    container(modes_row)
        .padding(6)
        .style(styles::card_style)
        .into()
}
