use gpui_kit::{
    Context, InteractiveElement, IntoElement, ParentElement, Styled,
    assets::IconName,
    base::h_flex,
    component::{
        ActiveTheme, Icon, Side,
        button::{Button, ButtonVariants},
        calendar::Calendar,
        sidebar::*,
    },
    div, px, relative,
};

use gpui_kit as gpui;

use crate::state::State;

pub fn sidebar(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    Sidebar::new(0)
        .w(px(300.))
        .h(relative(1.0))
        .border_r_1()
        .header(
            div()
                .w_full()
                .child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .p_2()
                        .child("春秋日历")
                        .child(
                            Button::new("settings")
                                .ghost()
                                .on_click(cx.listener(
                                    |state, _, _, _| {
                                        println!("settings clicked")
                                    },
                                ))
                                .child(IconName::Settings),
                        ),
                )
                .child(Calendar::new(state.mini_state()).w_full()),
        )
        .child(
            SidebarGroup::new("Navigation").child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("Dashboard")
                            .icon(IconName::LayoutDashboard)
                            .on_click(|_, _, _| {
                                println!("Dashboard clicked")
                            }),
                    )
                    .child(
                        SidebarMenuItem::new("Settings")
                            .icon(IconName::Settings)
                            .on_click(|_, _, _| {
                                println!("Settings clicked")
                            }),
                    ),
            ),
        )
        .footer(SidebarFooter::new().child("User Profile"))
}
