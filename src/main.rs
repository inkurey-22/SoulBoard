use iced::{
    Font, Task,
    font::{Family, Weight},
};

mod app;
mod bridge;
mod model;

use app::{Soulboard, update, view};
use bridge::start_bridge;
use model::TeamState;

const APP_FONT: Font = Font {
    family: Family::Name("Anek Gujarati Condensed SemiBold"),
    weight: Weight::Semibold,
    ..Font::DEFAULT
};

const APP_FONT_BYTES: &[u8] = include_bytes!("../assets/app-font.ttf");

fn main() -> iced::Result {
    iced::application("Soulboard", update, view)
        .default_font(APP_FONT)
        .font(APP_FONT_BYTES)
        .centered()
        .run_with(|| {
            let state = TeamState::default();
            (
                Soulboard {
                    state: state.clone(),
                    bridge: Some(start_bridge(state)),
                },
                Task::none(),
            )
        })
}
