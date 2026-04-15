use crate::app::palette as pal;
use iced::widget::{button, checkbox, container, text_input, scrollable, overlay::menu};
use iced::{Background, Border, Color, Vector, Shadow};

pub struct Card;
pub struct PrimaryButton;
pub struct Input;
pub struct Dropdown;
pub struct Checkbox;

impl container::Catalog for Card {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {
    }

    fn style(&self, _class: &Self::Class<'_>) -> container::Style {
        container::Style {
            background: Some(Background::Color(pal::mantle())),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
            text_color: None,
        }
    }
}

impl button::Catalog for PrimaryButton {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {
    }

    fn style(&self, _class: &Self::Class<'_>, status: button::Status) -> button::Style {
        let active = button::Style {
            background: Some(Background::Color(pal::red())),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 6.0.into(),
            },
            text_color: pal::base(),
            shadow: Shadow {
                offset: Vector::new(0.0, 0.0),
                ..Default::default()
            },
        };

        match status {
            button::Status::Active | button::Status::Pressed | button::Status::Disabled => active,
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(pal::peach())),
                ..active
            },
        }
    }
}

impl text_input::Catalog for Input {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {
    }

    fn style(&self, _class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        let active = text_input::Style {
            background: Background::Color(Color::from_rgb8(0xff, 0xff, 0xff)),
            border: Border {
                color: pal::surface1(),
                width: 1.0,
                radius: 6.0.into(),
            },
            icon: Color::TRANSPARENT,
            placeholder: pal::subtext0(),
            value: pal::text(),
            selection: pal::red(),
        };

        match status {
            text_input::Status::Active
            | text_input::Status::Hovered
            | text_input::Status::Focused
            | text_input::Status::Disabled => active,
        }
    }
}

impl scrollable::Catalog for Dropdown {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {
    }
    

    fn style(&self, _class: &Self::Class<'_>, _status: scrollable::Status) -> scrollable::Style {
        scrollable::Style {
            container: container::Style::default(),
            vertical_rail: scrollable::Rail {
                background: None,
                border: Border::default(),
                scroller: scrollable::Scroller {
                    color: Color::from_rgba8(0, 0, 0, 0.2),
                    border: Border::default(),
                },
            },
            horizontal_rail: scrollable::Rail {
                background: None,
                border: Border::default(),
                scroller: scrollable::Scroller {
                    color: Color::from_rgba8(0, 0, 0, 0.2),
                    border: Border::default(),
                },
            },
            gap: None,
        }
    }
}

impl menu::Catalog for Dropdown {
    type Class<'a> = ();

    fn default<'a>() -> <Self as menu::Catalog>::Class<'a> {
    }

    fn style(&self, _class: &<Self as menu::Catalog>::Class<'_>) -> menu::Style {
        menu::Style {
            text_color: pal::text(),
            background: Background::Color(pal::surface1()),
            border: Border {
                color: pal::surface1(),
                width: 1.0,
                radius: 6.0.into(),
            },
            selected_text_color: pal::base(),
            selected_background: Background::Color(pal::red()),
        }
    }
}

impl checkbox::Catalog for Checkbox {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {
    }

    fn style(&self, _class: &Self::Class<'_>, status: checkbox::Status) -> checkbox::Style {
        let is_checked = match status {
            checkbox::Status::Active { is_checked }
            | checkbox::Status::Hovered { is_checked }
            | checkbox::Status::Disabled { is_checked } => is_checked,
        };

        checkbox::Style {
            background: if is_checked {
                Background::Color(pal::red())
            } else {
                Background::Color(pal::surface1())
            },
            icon_color: pal::base(),
            border: Border {
                color: if is_checked {
                    pal::red()
                } else {
                    pal::surface1()
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Some(pal::text()),
        }
    }
}

// Convenience helpers to simplify usage sites in UI code. These are generic
// functions that forward to the Catalog implementations above so callers can
// pass a plain function pointer instead of repeating closure boilerplate.
pub fn card_style<Class>(_class: &Class) -> container::Style {
    container::Catalog::style(&Card, &())
}

pub fn primary_button_style<Class>(_class: &Class, status: button::Status) -> button::Style {
    button::Catalog::style(&PrimaryButton, &(), status)
}

pub fn input_style<Class>(_class: &Class, status: text_input::Status) -> text_input::Style {
    text_input::Catalog::style(&Input, &(), status)
}

pub fn dropdown_menu_style<Class>(_class: &Class) -> menu::Style {
    menu::Catalog::style(&Dropdown, &())
}

pub fn checkbox_style<Class>(_class: &Class, status: checkbox::Status) -> checkbox::Style {
    checkbox::Catalog::style(&Checkbox, &(), status)
}
