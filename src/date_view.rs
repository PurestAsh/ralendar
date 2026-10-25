use gpui_kit::{
    Context, IntoElement, ParentElement, Styled, Window, div,
};

use crate::state::State;

mod month_view;
mod year_view;

/// 年月日视图入口：年视图和月视图共用同一个内容区域。
pub fn main_content(
    state: &mut State,
    cx: &Context<State>,
    window: &mut Window,
) -> impl IntoElement {
    let mut content = div().size_full();

    if state.year_check() {
        content = content.child(year_view::year_view(state, cx));
    } else {
        content = content
            .child(month_view::main_content(state, cx, window));
    }

    content
}
