use gpui_kit::{
    self as gpui, Context, InteractiveElement, Window,
    base::{StyledExt, v_flex},
    px, relative,
};

use gpui::{
    IntoElement, ParentElement, Styled, component::TitleBar, div,
};

use crate::{
    icon::mini,
    main_content::main_content,
    side_bar::sidebar,
    state::State,
    style::{global_style, sidebar_style},
    title_bar::title_bar,
};

pub fn ui(
    state: &mut State,
    cx: &Context<State>,
    window: &mut Window,
) -> impl IntoElement {
    div()
        .refine_style(&global_style(cx))
        .flex()
        .w(relative(1.0))
        .h(relative(1.0))
        .child(sidebar(state, cx))
        .child(
            v_flex()
                .w_full()
                .h_full()
                .child(title_bar(state, cx))
                .child(
                    div()
                        .id("mian_content")
                        .w_full()
                        .flex_1()
                        .min_h(px(0.))
                        .child(main_content(state, cx, window)),
                ),
        )
}
