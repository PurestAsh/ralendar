use gpui_kit::base::{StyledExt, h_flex};
use gpui_kit::component::button::{
    Button, ButtonVariants, Toggle, ToggleGroup,
};
use gpui_kit::component::{ActiveTheme, IconName};
use gpui_kit::{self as gpui, Context, px, size};
use gpui_kit::{
    IntoElement, ParentElement, Styled, component::TitleBar, div,
    relative,
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
            .gap_1()
            .refine_style(&titlebar_style(cx))
            .child("2026年10月")
            .child(div().flex_grow_1())
            .child(
                ToggleGroup::new("date-switch")
                    .mr_2()
                    .my_1()
                    .rounded(cx.theme().radius)
                    .gap_0()
                    .border_1()
                    .border_color(cx.theme().border)
                    .child(
                        Toggle::new(0)
                            .label("年")
                            .checked(state.year_check()),
                    )
                    .child(
                        Toggle::new(1)
                            .label("月")
                            .checked(state.month_check()),
                    )
                    .child(
                        Toggle::new(2)
                            .label("日")
                            .checked(state.day_check()),
                    )
                    .on_click(cx.listener(
                        |state, checkeds, _, _| {
                            state.switch_mode(checkeds)
                        },
                    )),
            )
            .child(
                Button::new("today").secondary().child("今天").mx_2(),
            )
            .child(
                Button::new("left")
                    .ghost()
                    .icon(IconName::ChevronLeft),
            )
            .child(
                Button::new("right")
                    .ghost()
                    .icon(IconName::ChevronRight),
            )
            .child(
                Button::new("new-event")
                    .ghost()
                    .primary()
                    .size(px(25.))
                    .icon(IconName::Plus),
            ),
    )
}
