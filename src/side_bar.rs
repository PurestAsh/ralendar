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
    px, relative, rems,
};

use gpui_kit as gpui;

use crate::state::State;

pub fn sidebar(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    Sidebar::new(0)
        .w(px(288.))
        .h(relative(1.0))
        .border_r_1()
        .collapsible(SidebarCollapsible::Offcanvas)
        .collapsed(state.sidebar_switch())
        .header(
            div()
                .w_full()
                .flex()
                .flex_col()
                .child(div().p_2().child("Talendar"))
                .child(
                    // 日历的日期格是 rem 尺寸（每个 2rem），全局字号调大后
                    // 固定 248px 的默认宽度已经装不下一周了。这里按 7 个
                    // 日期格计算宽度，并在侧边栏里整体居中。
                    div().w_full().flex().justify_center().child(
                        Calendar::new(state.mini_state())
                            .w(rems(14.0))
                            .px(px(0.))
                            .py_2()
                            .border_0(),
                    ),
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
