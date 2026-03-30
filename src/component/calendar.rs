use crate::style::{background, light_gray};
use crate::{message::Message, state::ApplicationState};
use iced::alignment::Vertical;
use iced::widget::{Space, column, container, row, text};
use iced::{Color, Element, Length, Padding};

#[derive(Default)]
pub struct CalendarState {
    pub title_offset: f32,
}

pub fn view<'a>(state: &ApplicationState) -> Element<'a, Message> {
    container(
        row![
            column![
                container(Space::new())
                    .width(Length::Fill)
                    .height(Length::Fill)
            ]
            .width(10)
            .height(Length::Fill),
            column![
                container(header(state))
                    .width(Length::Fill)
                    .height(70)
                    .padding([20, 0]),
                container(Space::new().width(Length::Fill).height(Length::Fill))
                    .style(|_theme| { background(Color::BLACK) })
            ]
            .padding(Padding::ZERO.bottom(10))
            .width(Length::Fill)
            .height(Length::Fill),
            Space::new().width(10).height(Length::Fill)
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| background(light_gray()))
    .into()
}

pub fn header<'a>(state: &ApplicationState) -> Element<'a, Message> {
    row![
        // Space::new().width(25),
        Space::new().width(state.calendar_state.title_offset),
        text!("九月").size(18)
    ]
    .height(Length::Fill)
    .width(Length::Fill)
    .align_y(Vertical::Center)
    .into()
}
