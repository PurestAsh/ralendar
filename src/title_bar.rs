use chrono::{Datelike, Duration, NaiveDate};
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{
    Button, ButtonVariants, Toggle, ToggleGroup,
};
use gpui_kit::component::sidebar::SidebarToggleButton;
use gpui_kit::component::{ActiveTheme, IconName};
use gpui_kit::{Context, px};
use gpui_kit::{
    IntoElement, ParentElement, Styled, component::TitleBar, div,
};

use crate::dialog::new_event;
use crate::style::titlebar_style;

use crate::state::State;

fn days_in_month(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    let first = NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .expect("invalid month");
    first.pred_opt().expect("invalid date").day()
}

fn shift_month(state: &mut State, delta: i32) {
    let current = *state.select_date();
    let total = current.year() * 12 + current.month0() as i32 + delta;
    let year = total.div_euclid(12);
    let month = total.rem_euclid(12) as u32 + 1;
    let day = current.day().min(days_in_month(year, month));

    state.change_select_date(
        NaiveDate::from_ymd_opt(year, month, day)
            .expect("invalid date"),
    );
}

fn shift_year(state: &mut State, delta: i32) {
    let current = *state.select_date();
    let year = current.year() + delta;
    // 2 月 29 日在平年里没有对应日期，退到 2 月 28 日。
    let date = current.with_year(year).unwrap_or_else(|| {
        NaiveDate::from_ymd_opt(year, current.month(), 28)
            .expect("invalid date")
    });
    state.change_select_date(date);
}

/// 上一页/下一页跟随当前视图：年视图翻年、日视图翻天、月视图翻月。
fn move_page(state: &mut State, delta: i32) {
    if state.year_check() {
        shift_year(state, delta);
    } else if state.day_check() {
        let date =
            *state.select_date() + Duration::days(delta as i64);
        state.change_select_date(date);
    } else {
        shift_month(state, delta);
    }
}

pub fn title_bar(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    div().w_full().child(
        TitleBar::new()
            .flex()
            .gap_1()
            .refine_style(&titlebar_style(cx))
            .child(
                div().relative().left(px(-6.)).child(
                    SidebarToggleButton::new()
                        .collapsed(state.sidebar_switch())
                        .on_click(cx.listener(|state, _, _, _| {
                            state.toggle_sidebar();
                        })),
                ),
            )
            .child(
                div()
                    .pl_2()
                    .border_l_1()
                    .border_color(cx.theme().border)
                    .child({
                        let date = state.select_date();
                        if state.year_check() {
                            format!("{}年", date.year())
                        } else if state.day_check() {
                            format!(
                                "{}年{}月{}日",
                                date.year(),
                                date.month(),
                                date.day()
                            )
                        } else {
                            format!(
                                "{}年{}月",
                                date.year(),
                                date.month()
                            )
                        }
                    }),
            )
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
                        |state, checkeds, _, cx| {
                            state.switch_mode(checkeds);
                            cx.notify();
                        },
                    )),
            )
            .child(
                Button::new("today")
                    .secondary()
                    .child("今天")
                    .mx_2()
                    .on_click(cx.listener(|state, _, _, cx| {
                        let today = *state.today();
                        state.change_select_date(today);
                        cx.notify();
                    })),
            )
            .child(
                Button::new("left")
                    .ghost()
                    .icon(IconName::ChevronLeft)
                    .on_click(cx.listener(|state, _, _, cx| {
                        move_page(state, -1);
                        cx.notify();
                    })),
            )
            .child(
                Button::new("right")
                    .ghost()
                    .icon(IconName::ChevronRight)
                    .on_click(cx.listener(|state, _, _, cx| {
                        move_page(state, 1);
                        cx.notify();
                    })),
            )
            .child(
                Button::new("new-event")
                    .ghost()
                    .primary()
                    .size(px(25.))
                    .on_click(|_, window, cx| {
                        new_event(window, cx);
                    })
                    .icon(IconName::Plus),
            ),
    )
}
