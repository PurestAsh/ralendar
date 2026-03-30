use crate::{message::Message, state::ApplicationState};
use iced::Element;
use iced::widget::container;
use iced::widget::container::background;
use iced::{Color, Length};

pub fn view<'a>(_state: &ApplicationState) -> Element<'a, Message> {
    container("")
        .height(Length::Fill)
        .width(300)
        .style(|_theme| background(Color::from_rgb(0.1, 0.1, 0.1)))
        .into()
}
