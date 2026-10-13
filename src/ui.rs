use gpui_kit::{
    self as gpui, Context, base::StyledExt, px, relative,
};

use gpui::{
    IntoElement, ParentElement, Styled, component::TitleBar, div,
};

use crate::{
    icon::mini,
    side_bar::sidebar,
    state::State,
    style::{global_style, sidebar_style},
    title_bar::title_bar,
};

pub fn ui(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    div()
        .refine_style(&global_style(cx))
        .flex()
        .w(relative(1.0))
        .h(relative(1.0))
        .child(sidebar(state, cx))
        .child(title_bar(state, cx))
}
