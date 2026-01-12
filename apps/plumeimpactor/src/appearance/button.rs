use iced::widget::button;
use iced::{Background, Border, Color, Shadow, Theme};

use super::{THEME_CORNER_RADIUS, darken, lighten};

/// Primary button style
pub fn p_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.palette();

    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(palette.primary)),
            text_color: palette.background,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(lighten(palette.primary, 0.15))),
            text_color: lighten(palette.background, 0.1),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(lighten(palette.primary, 0.03))),
            text_color: darken(palette.background, 0.1),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(
                lighten(palette.primary, 0.05).scale_alpha(0.2),
            )),
            text_color: palette.background.scale_alpha(0.5),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

/// Secondary button style
pub fn s_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.palette();

    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(lighten(palette.background, 0.03))),
            text_color: palette.text,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(lighten(palette.background, 0.15))),
            text_color: lighten(palette.text, 0.1),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(lighten(palette.background, 0.03))),
            text_color: darken(palette.text, 0.1),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(lighten(palette.background, 0.05))),
            text_color: palette.text.scale_alpha(0.5),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
    }
}
