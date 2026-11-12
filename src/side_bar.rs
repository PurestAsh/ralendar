use gpui_kit::{
    Context, IntoElement, ParentElement, Styled,
    base::h_flex,
    component::{
        Icon, IconName,
        calendar::Calendar,
        sidebar::{
            Sidebar, SidebarCollapsible, SidebarFooter, SidebarGroup,
            SidebarMenu, SidebarMenuItem,
        },
    },
    div,
    prelude::FluentBuilder,
    px, relative, rems,
};

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
        .child(SidebarGroup::new("我的日历").child(
            SidebarMenu::new().children(
                state.schedules().iter().enumerate().map(
                    |(pos, cat)| {
                        SidebarMenuItem::new(&cat.name)
                            .icon(Icon::empty())
                            .when(cat.enabled, |a| {
                                a.icon(IconName::Check)
                            })
                            .on_click(cx.listener(
                                move |state, _, _, _| {
                                    state.toggle_schedule(pos);
                                },
                            ))
                    },
                ),
            ),
        ))
        .footer(
            SidebarFooter::new().justify_between().child(
                h_flex()
                    .gap_2()
                    .child(IconName::CircleUser)
                    .child("Three Water"),
            ),
        )
}
