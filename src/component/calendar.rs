use crate::message::{ContentMsg, DateType};
use crate::style::{background, black, deep_gray, grau, light_gray, purple};
use crate::{message::Message, state::ApplicationState};
use iced::alignment::Vertical;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::horizontal;
use iced::widget::{Grid, Space, button, column, container, row, stack, text};
use iced::{Alignment, Animation, Border, Color, Element, Length, Padding, Task};
use lilt::Easing;
use std::time::{Duration, Instant};

pub struct CalendarState {
    pub title_offset: f32,
    pub animation: Animation<DateType>,
    pub date_offset: f32,
    pub current_date_type: DateType,
}

impl Default for CalendarState {
    fn default() -> Self {
        Self {
            title_offset: 40.,
            animation: Animation::new(DateType::Month)
                .easing(Easing::EaseOutQuint)
                .duration(Duration::from_millis(500)),
            date_offset: 40.,
            current_date_type: DateType::Month,
        }
    }
}

fn black_line() -> Element<'static, Message> {
    container(Space::new())
        .height(2)
        .width(Length::Fill)
        .style(|_theme| ContainerStyle {
            background: Some(black().into()),
            ..Default::default()
        })
        .into()
}

fn vertical_divider() -> Element<'static, Message> {
    container(Space::new())
        .height(Length::Fill)
        .width(2)
        .style(|_theme| ContainerStyle {
            background: Some(black().into()),
            ..Default::default()
        })
        .into()
}

pub fn view<'a>(state: &ApplicationState) -> Element<'a, Message> {
    container(
        row![
            column![
                Space::new().height(91),
                container(column![text("1"), text("周")].align_x(Alignment::Center))
                    .align_top(Length::FillPortion(1))
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
                container(column![text("2"), text("周")].align_x(Alignment::Center))
                    .align_top(Length::FillPortion(1))
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
                container(column![text("3"), text("周")].align_x(Alignment::Center))
                    .align_top(Length::FillPortion(1))
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
                container(column![text("4"), text("周")].align_x(Alignment::Center))
                    .align_top(Length::FillPortion(1))
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
                container(column![text("5"), text("周")].align_x(Alignment::Center))
                    .align_top(Length::FillPortion(1))
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
                Space::new().height(15)
            ]
            .width(15)
            .height(Length::Fill),
            column![
                container(header(state))
                    .width(Length::Fill)
                    .height(70)
                    .padding([20, 0]),
                container(main_content(state))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(|_theme| {
                        ContainerStyle {
                            border: Border {
                                radius: 8.0.into(),
                                ..Default::default()
                            },
                            background: Some(light_gray().into()),
                            ..Default::default()
                        }
                    })
            ]
            .padding(Padding::ZERO.bottom(10))
            .width(Length::Fill)
            .height(Length::Fill),
            Space::new().width(15).height(Length::Fill)
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .style(|_theme| background(deep_gray()))
    .into()
}

pub fn header<'a>(state: &ApplicationState) -> Element<'a, Message> {
    stack!(
        row![
            Space::new().width(state.calendar_state.title_offset),
            text!("九月").size(18)
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        .align_y(Vertical::Center),
        row![
            horizontal(),
            row![
                Space::new().width(state.calendar_state.date_offset),
                container(Space::new())
                    .width(40)
                    .height(30)
                    .style(|_theme| {
                        ContainerStyle {
                            border: Border {
                                radius: 15.0.into(),
                                ..Default::default()
                            },
                            background: Some(purple().into()),
                            ..Default::default()
                        }
                    }),
                horizontal()
            ]
            .width(120),
            horizontal()
        ]
        .width(Length::Fill)
        .height(Length::Fill),
        row![
            horizontal(),
            row![
                button(
                    text!("年")
                        .width(40)
                        .height(30)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                )
                .on_press(Message::ContentMsg(ContentMsg::Header(DateType::Year)))
                .padding(Padding::ZERO)
                .style(|_thme, _status| ButtonStyle::default()),
                button(
                    text!("月")
                        .width(40)
                        .height(30)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                )
                .on_press(Message::ContentMsg(ContentMsg::Header(DateType::Month)))
                .padding(Padding::ZERO)
                .style(|_thme, _status| ButtonStyle::default()),
                button(
                    text!("日")
                        .width(40)
                        .height(30)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                )
                .on_press(Message::ContentMsg(ContentMsg::Header(DateType::Day)))
                .padding(Padding::ZERO)
                .style(|_thme, _status| ButtonStyle::default()),
            ]
            .width(120)
            .height(30),
            horizontal()
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .into()
}

fn day_cell<'a>(day: &crate::backend::Day, x: isize) -> Element<'a, Message> {
    column![
        if x < 7 {
            Space::new().into()
        } else {
            black_line()
        },
        row![
            column![
                row![
                    text(format!("{}", day.day())),
                    horizontal(),
                    text(format!("{}", day.lunar()))
                ]
                .width(Length::Fill)
                .height(15),
                Space::new().height(5),
                container(
                    row![
                        container(Space::new())
                            .height(Length::Fill)
                            .width(2)
                            .style(|_| ContainerStyle {
                                background: Some(Color::BLACK.into()),
                                ..Default::default()
                            }),
                        horizontal(),
                        text(format!("{}", day.event())),
                    ]
                    .width(Length::Fill)
                    .height(20)
                )
                .style(|_theme| {
                    ContainerStyle {
                        background: Some(grau().into()),
                        ..Default::default()
                    }
                }),
            ].width(Length::Fill)
            .padding(2),
            if (x - 6) % 7 == 0 {
                Space::new().into()
            } else {
                vertical_divider()
            }
        ].height(Length::Fill)
        .width(Length::Fill)
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

pub fn main_content<'a>(state: &ApplicationState) -> Element<'a, Message> {
    let mut cells = Vec::new();
    let data = state.calendar.get_cache();
    if data.is_none() {
        return "".into();
    }
    let mut iter = data.unwrap().iter();
    for x in 0..42 {
        let day = iter.next().unwrap();
        cells.push(container(day_cell(&day, x)).into());
    }

    column![
        row![
            container("日")
                .width(Length::Fill)
                .height(Length::FillPortion(1))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            container("一")
                .width(Length::Fill)
                .height(Length::FillPortion(1))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            container("二")
                .width(Length::Fill)
                .height(Length::FillPortion(1))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            container("三")
                .width(Length::Fill)
                .height(Length::FillPortion(1))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            container("四")
                .width(Length::Fill)
                .height(Length::FillPortion(1))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            container("五")
                .width(Length::Fill)
                .height(Length::FillPortion(1))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            container("六")
                .width(Length::Fill)
                .height(Length::FillPortion(1))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        ]
        .width(Length::Fill)
        .height(25)
        .align_y(Alignment::Center)
        .padding(Padding::ZERO.bottom(5)),
        container(Space::new())
            .width(Length::Fill)
            .height(2)
            .style(|_theme| {
                ContainerStyle {
                    background: Some(black().into()),
                    ..Default::default()
                }
            }),
        Space::new().height(4),
        Grid::from_vec(cells).height(Length::Fill).columns(7)
    ]
    .into()
}

pub fn update(state: &mut ApplicationState, message: ContentMsg) -> Task<Message> {
    match message {
        ContentMsg::Header(date_type) => {
            if date_type != state.calendar_state.current_date_type {
                state.calendar_state.current_date_type = date_type;
                Task::done(Message::ContentMsg(ContentMsg::Animation))
            } else {
                Task::none()
            }
        }
        ContentMsg::Animation => {
            let now = Instant::now();
            let old_date_type = state.calendar_state.animation.value();
            let current_date_type = state.calendar_state.current_date_type;
            if old_date_type != current_date_type
                || state.calendar_state.animation.is_animating(now)
            {
                state
                    .calendar_state
                    .animation
                    .go_mut(current_date_type, now);

                let date_offset = state.calendar_state.animation.interpolate_with(
                    |date_type| match date_type {
                        DateType::Day => 80.,
                        DateType::Month => 40.,
                        DateType::Year => 0.,
                    },
                    now,
                );
                state.calendar_state.date_offset = date_offset;
                Task::done(Message::ContentMsg(ContentMsg::Animation))
            } else {
                Task::none()
            }
        }
    }
}
