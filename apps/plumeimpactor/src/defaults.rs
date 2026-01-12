use iced::Length;
use iced::widget::text;
use iced::widget::text::LineHeight;

pub fn load() -> Vec<std::borrow::Cow<'static, [u8]>> {
    vec![include_bytes!("./halloy-icons.ttf").as_slice().into()]
}

pub(crate) fn default_settings() -> iced::Settings {
    iced::Settings {
        default_font: crate::appearance::p_font(),
        default_text_size: crate::appearance::THEME_FONT_SIZE.into(),
        fonts: load(),
        ..Default::default()
    }
}

pub(crate) fn default_window_settings() -> iced::window::Settings {
    #[cfg(target_os = "macos")]
    let platform_specific = iced::window::settings::PlatformSpecific {
        titlebar_transparent: true,
        title_hidden: true,
        fullsize_content_view: true,
        ..Default::default()
    };

    #[cfg(not(target_os = "macos"))]
    let platform_specific = iced::window::settings::PlatformSpecific::default();

    iced::window::Settings {
        size: iced::Size::new(555.0, 300.0),
        position: iced::window::Position::Centered,
        exit_on_close_request: false,
        resizable: false,
        icon: Some(load_window_icon()),
        platform_specific: platform_specific,
        ..Default::default()
    }
}

fn load_window_icon() -> iced::window::Icon {
    let bytes = include_bytes!(
        "../../../package/linux/icons/hicolor/64x64/apps/dev.khcrysalis.PlumeImpactor.png"
    );
    let image = image::load_from_memory(bytes)
        .expect("Failed to load icon bytes")
        .to_rgba8();
    let (width, height) = image.dimensions();
    iced::window::icon::from_rgba(image.into_raw(), width, height).unwrap()
}

pub fn get_data_path() -> std::path::PathBuf {
    let base = if cfg!(windows) {
        std::env::var("APPDATA").unwrap()
    } else {
        std::env::var("HOME").unwrap() + "/.config"
    };

    let dir = std::path::Path::new(&base).join("PlumeImpactor");

    std::fs::create_dir_all(&dir).ok();

    dir
}

pub type Text<'a> = iced::widget::Text<'a, iced::Theme, iced::Renderer>;

pub fn dot<'a>() -> Text<'a> {
    to_text('\u{F111}')
}

pub fn error<'a>() -> Text<'a> {
    to_text('\u{E80D}')
}

pub fn connected<'a>() -> Text<'a> {
    to_text('\u{E800}')
}

pub fn link() -> Text<'static> {
    to_text('\u{E814}')
}

pub fn cancel<'a>() -> Text<'a> {
    to_text('\u{E80F}')
}

pub fn maximize<'a>() -> Text<'a> {
    to_text('\u{E801}')
}

pub fn restore<'a>() -> Text<'a> {
    to_text('\u{E805}')
}

pub fn people<'a>() -> Text<'a> {
    to_text('\u{E804}')
}

pub fn topic<'a>() -> Text<'a> {
    to_text('\u{E803}')
}

pub fn search<'a>() -> Text<'a> {
    to_text('\u{E808}')
}

pub fn checkmark<'a>() -> Text<'a> {
    to_text('\u{E806}')
}

pub fn file_transfer<'a>() -> Text<'a> {
    to_text('\u{E802}')
}

pub fn refresh<'a>() -> Text<'a> {
    to_text('\u{E807}')
}

pub fn megaphone<'a>() -> Text<'a> {
    to_text('\u{E809}')
}

pub fn theme_editor<'a>() -> Text<'a> {
    to_text('\u{E80A}')
}

pub fn undo<'a>() -> Text<'a> {
    to_text('\u{E80B}')
}

pub fn copy<'a>() -> Text<'a> {
    to_text('\u{F0C5}')
}

pub fn popout<'a>() -> Text<'a> {
    to_text('\u{E80E}')
}

pub fn logs<'a>() -> Text<'a> {
    to_text('\u{E810}')
}

pub fn menu<'a>() -> Text<'a> {
    to_text('\u{F0C9}')
}

pub fn documentation<'a>() -> Text<'a> {
    to_text('\u{E812}')
}

pub fn highlights<'a>() -> Text<'a> {
    to_text('\u{E811}')
}

pub fn scroll_to_bottom<'a>() -> Text<'a> {
    to_text('\u{F103}')
}

pub fn share<'a>() -> Text<'a> {
    to_text('\u{E813}')
}

pub fn mark_as_read<'a>() -> Text<'a> {
    to_text('\u{E817}')
}

pub fn config<'a>() -> Text<'a> {
    to_text('\u{F1C9}')
}

pub fn star<'a>() -> Text<'a> {
    to_text('\u{E819}')
}

pub fn certificate<'a>() -> Text<'a> {
    to_text('\u{F0A3}')
}

pub fn circle_empty<'a>() -> Text<'a> {
    to_text('\u{F10C}')
}

pub fn dot_circled<'a>() -> Text<'a> {
    to_text('\u{F192}')
}

pub fn asterisk<'a>() -> Text<'a> {
    to_text('\u{E815}')
}

pub fn speaker<'a>() -> Text<'a> {
    to_text('\u{E818}')
}

pub fn lightbulb<'a>() -> Text<'a> {
    to_text('\u{F0EB}')
}

pub fn quit<'a>() -> Text<'a> {
    to_text('\u{F02D}')
}

pub fn channel_discovery<'a>() -> Text<'a> {
    to_text('\u{E81D}')
}
pub const ICON_SIZE: f32 = 12.0;
pub const ICON: iced::Font = iced::Font::with_name("halloy-icons");
fn to_text<'a>(unicode: char) -> Text<'a> {
    text(unicode.to_string())
        .line_height(LineHeight::Relative(1.0))
        .size(ICON_SIZE)
        .font(ICON)
}
