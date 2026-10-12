use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{
    Toggle, ToggleGroup,
};
use gpui_kit::{self as gpui, Context};
use gpui_kit::{
    IntoElement, ParentElement, Styled,
    component::TitleBar, div, relative,
};

use crate::style::titlebar_style;

use crate::state::State;
pub fn title_bar(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    div().w(relative(1.0)).child(
        TitleBar::new()
            .flex()
            .refine_style(&titlebar_style(cx))
            .child(div().flex_grow_1())
            .child(
                ToggleGroup::new("date-switch")
                    .child(
                        Toggle::new(0)
                            .label("年")
                            .checked(
                                state.year_check(),
                            ),
                    )
                    .child(
                        Toggle::new(1)
                            .label("月")
                            .checked(
                                state.month_check(),
                            ),
                    )
                    .child(
                        Toggle::new(2)
                            .label("日")
                            .checked(
                                state.day_check(),
                            ),
                    )
                    .on_click(cx.listener(
                        |state, checkeds, _, _| {
                            state.switch_date(checkeds)
                        },
                    )),
            ),
    )
}
