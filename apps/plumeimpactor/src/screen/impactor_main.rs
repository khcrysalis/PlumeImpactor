use iced::widget::{column, container, text};
use iced::{Center, Color, Element};

use crate::Message;

pub fn view(_is_hovering: bool) -> Element<'static, Message> {
    let content = column![
        text("Drag & drop IPA / TIPA file")
            .size(16)
            .color(Color::from_rgba(0.5, 0.5, 0.5, 0.7))
    ]
    .padding(20)
    .spacing(20)
    .align_x(Center);

    container(content).into()
}
