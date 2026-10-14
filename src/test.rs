use gpui_kit::{
    IntoElement, ParentElement, Styled,
    component::{TitleBar, calendar::Calendar},
    div, px, relative,
};

use crate::state::State;

pub fn test(state: &mut State) -> impl IntoElement {
    div()
        .flex()
        .border_1()
        .bg(gpui_kit::blue())
        .child(Calendar::new(state.mini_state()).border_0())
}
