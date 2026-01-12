use iced::Theme;
use iced::color;

pub mod button;
pub mod picklist;

pub use button::p_button;
pub use picklist::primary_pick_list;

pub const THEME_CORNER_RADIUS: f32 = 4.0;
pub const THEME_FONT_SIZE: f32 = 12.0;

pub fn p_font() -> iced::Font {
    iced::Font {
        family: iced::font::Family::Monospace,
        weight: iced::font::Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlumeTheme {
    PlumeDark,
}

impl PlumeTheme {
    pub fn to_iced_theme(self) -> Theme {
        Self::plume_dark()
    }

    fn plume_dark() -> Theme {
        Theme::custom(
            "Plume Dark".to_string(),
            iced::theme::Palette {
                background: color!(0x282021),
                text: color!(0xf2d5cf),
                primary: color!(0xd3869b),
                success: color!(0xd9a6b3),
                danger: color!(0xe78a8a),
                warning: color!(0xf4b8c4),
            },
        )
    }
}

impl std::fmt::Display for PlumeTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plume Dark")
    }
}

impl Default for PlumeTheme {
    fn default() -> Self {
        Self::PlumeDark
    }
}
