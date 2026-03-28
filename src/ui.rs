use crate::component::calendar;
use crate::component::schedule;
use crate::component::side_bar;
use crate::{message::Message, state::ApplicationState};
use iced::Element;
use iced::Length;
use iced::widget::row;

pub fn main_page<'a>(state: &ApplicationState) -> Element<'a, Message> {
    row([
        side_bar::view(state),
        calendar::view(state),
        schedule::view(state),
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
