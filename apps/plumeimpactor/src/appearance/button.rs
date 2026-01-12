use iced::widget::button;
use iced::{Background, Border, Color, Shadow, Theme};

use super::THEME_CORNER_RADIUS;

pub fn p_button(theme: &Theme, status: button::Status) -> button::Style {
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

fn lighten(color: Color, amount: f32) -> Color {
    Color {
        r: (color.r + amount).min(1.0),
        g: (color.g + amount).min(1.0),
        b: (color.b + amount).min(1.0),
        a: color.a,
    }
}

fn darken(color: Color, amount: f32) -> Color {
    Color {
        r: (color.r - amount).max(0.0),
        g: (color.g - amount).max(0.0),
        b: (color.b - amount).max(0.0),
        a: color.a,
    }
}
