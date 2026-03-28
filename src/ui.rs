use crate::component::calendar;
use crate::component::schedule;
use crate::component::side_bar;
use crate::{message::Message, state::ApplicationState};
use iced::Element;
use iced::Length;
use iced::widget::Stack;
use iced::widget::row;
use iced::widget::stack;

pub fn main_page<'a>(state: &ApplicationState) -> Element<'a, Message> {
    stack![
        row([
            side_bar::mask_view(state),
            calendar::view(state),
            schedule::view(state),
        ])
        .width(Length::Fill)
        .height(Length::Fill),
        side_bar::view(state),
    ]
    .into()
}
