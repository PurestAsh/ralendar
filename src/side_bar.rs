use gpui_kit::{
    Context, InteractiveElement, IntoElement, Length, ParentElement,
    Styled,
    assets::IconName,
    base::h_flex,
    component::{
        ActiveTheme, Icon, Side,
        button::{Button, ButtonVariants},
        calendar::Calendar,
        sidebar::*,
    },
    div,
    prelude::FluentBuilder,
    px, relative,
};

use gpui_kit as gpui;

use crate::state::State;

pub fn sidebar(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    Sidebar::new(0)
        .h(relative(1.0))
        .border_r_1()
        .collapsible(SidebarCollapsible::Offcanvas)
        .collapsed(state.sidebar_switch())
        .header(
            div()
                .relative()
                .child(div().p_2().child("春秋日历"))
                .child(
                    Calendar::new(state.mini_state())
                        .p_2()
                        .relative()
                        .left_neg_2()
                        .border_0(),
                ),
        )
        .child(
            SidebarGroup::new("我的日历").child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("我的日程")
                            .icon(Icon::empty())
                            .when(state.my_schedule(), |a| {
                                a.icon(IconName::Check)
                            })
                            .on_click(cx.listener(
                                |state, _, _, _| {
                                    state.toggle_schedule(0);
                                },
                            )),
                    )
                    .child(
                        SidebarMenuItem::new("中国节假日")
                            .icon(Icon::empty())
                            .when(state.cn_schedule(), |a| {
                                a.icon(IconName::Check)
                            })
                            .on_click(cx.listener(
                                |state, _, _, _| {
                                    state.toggle_schedule(1);
                                },
                            )),
                    )
                    .child(
                        SidebarMenuItem::new("国际节假日")
                            .icon(Icon::empty())
                            .when(state.en_schedule(), |a| {
                                a.icon(IconName::Check)
                            })
                            .on_click(cx.listener(
                                |state, _, _, _| {
                                    state.toggle_schedule(2);
                                },
                            )),
                    ),
            ),
        )
        .footer(
            SidebarFooter::new().justify_between().child(
                h_flex()
                    .gap_2()
                    .child(IconName::CircleUser)
                    .child("Three Water"),
            ),
        )
}
