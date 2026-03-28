use std::time::{Duration, Instant};

use crate::message::SideBarMsg;
use crate::style::{light_blue, light_orange, with_border};
use crate::widget::tr_container::TrContainer;
use crate::{message::Message, state::ApplicationState};
use iced::alignment::Vertical;
use iced::animation::Easing;
use iced::font::Weight;
use iced::widget::space::horizontal;
use iced::widget::svg::Handle;
use iced::widget::{Space, column, container, row, svg, text};
use iced::{Alignment, Animation, Font, Length, Padding};
use iced::{Element, Task};

const SIDE_BAR_WIDTH: f32 = 250.;

pub struct SideBarState {
    pub animation: Animation<bool>,
    pub width: f32,
    pub expand: bool,
}

impl Default for SideBarState {
    fn default() -> Self {
        Self {
            animation: Animation::new(true)
                .easing(Easing::EaseOutQuint)
                .duration(Duration::from_secs(1)),
            width: SIDE_BAR_WIDTH,
            expand: true,
        }
    }
}

pub fn mask_view<'a>(state: &ApplicationState) -> Element<'a, Message> {
    Space::new()
        .width(state.sidebar_state.width)
        .height(Length::Fill)
        .into()
}

pub fn view<'a>(state: &ApplicationState) -> Element<'a, Message> {
    TrContainer::new(
        container(column![
            row![
                svg(Handle::from_memory(include_bytes!(
                    "../../assets/icon/ralendar.svg"
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
                    "../../assets/icon/ic_cursor_left.svg"
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
        .width(SIDE_BAR_WIDTH)
        .padding([20, 20])
        .style(|theme| with_border(light_blue())),
    )
    .width(state.sidebar_state.width)
    .into()
}

pub fn update(state: &mut ApplicationState, message: SideBarMsg) -> Task<Message> {
    match message {
        SideBarMsg::Toggle => toggle_update(state),
        SideBarMsg::Animation => animation_update(state),
    }
}

fn toggle_update(state: &mut ApplicationState) -> Task<Message> {
    let sidbar_state = &mut state.sidebar_state;
    sidbar_state.expand = !sidbar_state.expand;
    if sidbar_state.expand != sidbar_state.animation.value() {
        let now = Instant::now();
        sidbar_state.animation.go_mut(sidbar_state.expand, now);
        Task::done(Message::SideBarMsg(SideBarMsg::Animation))
    } else {
        Task::none()
    }
}
fn animation_update(state: &mut ApplicationState) -> Task<Message> {
    let now = Instant::now();
    let animation = &state.sidebar_state.animation;
    if animation.is_animating(now) {
        let width = animation.interpolate(0., SIDE_BAR_WIDTH, now);
        state.sidebar_state.width = width;
        Task::done(Message::SideBarMsg(SideBarMsg::Animation))
    } else {
        Task::none()
    }
}
