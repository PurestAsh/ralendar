use gpui_kit::{
    self as gpui, Context, base::StyledExt, px,
    relative,
};

use gpui::{
    IntoElement, ParentElement, Styled,
    component::TitleBar, div,
};

use crate::{
    icon::mini, state::State,
    style::{global_style, sidebar},
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
        .child(
            div()
                .w(px(300.))
                .h(relative(1.0))
                .refine_style(&sidebar(cx))
                .border_r_1(),
        )
        .child(title_bar(state, cx))
}
