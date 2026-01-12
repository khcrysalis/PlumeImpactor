use iced::widget::pick_list;
use iced::{Background, Border, Color, Theme};

use super::THEME_CORNER_RADIUS;

pub fn primary_pick_list(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let palette = theme.palette();

    match status {
        pick_list::Status::Active => pick_list::Style {
            text_color: palette.text,
            placeholder_color: Color::from_rgb(0.6, 0.6, 0.65),
            handle_color: palette.text,
            background: Background::Color(lighten(palette.background, 0.03)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
        },
        pick_list::Status::Hovered => pick_list::Style {
            text_color: palette.text,
            placeholder_color: Color::from_rgb(0.7, 0.7, 0.75),
            handle_color: lighten(palette.text, 0.15),
            background: Background::Color(lighten(palette.background, 0.12)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
        },
        pick_list::Status::Opened { .. } => pick_list::Style {
            text_color: palette.text,
            placeholder_color: Color::from_rgb(0.7, 0.7, 0.75),
            handle_color: palette.text,
            background: Background::Color(lighten(palette.background, 0.10)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: THEME_CORNER_RADIUS.into(),
            },
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
