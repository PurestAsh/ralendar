use crate::style::{blue, light_blue, light_orange, with_border};
use crate::widget::SideBar;
use crate::{message::Message, state::ApplicationState};
use iced::alignment::Vertical;
use iced::font::Weight;
use iced::widget::space::horizontal;
use iced::widget::svg::Handle;
use iced::widget::{Space, button, column, container, row, svg, text};
use iced::{Alignment, Font, Length, Padding};
use iced::{Color, Element, widget::container::background};

pub fn main_page<'a>(state: &ApplicationState) -> Element<'a, Message> {
    row([side_bar(state), calender_view(state), schedule_vew(state)])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn side_bar<'a>(state: &ApplicationState) -> Element<'a, Message> {
    container(
        SideBar::new(
            container(column![
                row![
                    svg(Handle::from_memory(include_bytes!(
                        "../assets/icon/ralendar.svg"
                    )))
                    .height(30)
                    .width(30),
                    Space::new().width(10),
                    text("日历")
                        .font(Font {
                            weight: Weight::Bold,
                            ..Default::default()
                        })
                        .align_y(Vertical::Center),
                    horizontal(),
                    svg(Handle::from_memory(include_bytes!(
                        "../assets/icon/ic_cursor_left.svg"
                    )))
                    .height(25)
                    .width(25)
                ]
                .padding(Padding::default().bottom(20))
                .align_y(Vertical::Center),
                row![
                    text("日历账户").font(Font {
                        weight: Weight::Bold,
                        ..Default::default()
                    }),
                    horizontal(),
                    text("添加").color(light_orange())
                ]
                .align_y(Alignment::Center)
            ])
            .height(Length::Fill)
            .width(250)
            .padding([20, 20])
            .style(|theme| with_border(light_blue())),
        )
        .expand(state.sidebar_state.expand)
        .height(Length::Fill)
        .width(250.into()),
    )
    .height(Length::Fill)
    .width(250)
    .into()
}

fn calender_view<'a>(state: &ApplicationState) -> Element<'a, Message> {
    container(button("press").on_press(Message::ToggleSideBar))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn schedule_vew<'a>(stae: &ApplicationState) -> Element<'a, Message> {
    container("")
        .height(Length::Fill)
        .width(300)
        .style(|theme| background(Color::from_rgb(0.1, 0.1, 0.1)))
        .into()
}
