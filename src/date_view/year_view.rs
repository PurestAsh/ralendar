use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDate};

use gpui_kit::{
    Context, InteractiveElement, IntoElement, ParentElement, Styled,
    base::{StyledExt, h_flex, v_flex},
    component::ActiveTheme,
    div, px, rgb,
};

use crate::{backend::Event, state::State};

/// 年视图里每个月的星期表头，从周一开始。
const WEEKDAYS: [&str; 7] =
    ["一", "二", "三", "四", "五", "六", "日"];

/// 一天里需要显示的标记。年视图只展示普通日程的颜色，不显示标题。
#[derive(Clone, Copy)]
struct DayMark {
    color: u32,
}

/// 节假日不进入年视图，只保留个人日程。
fn is_holiday(event: &Event) -> bool {
    event.category_name.contains("节假日")
}

/// 一次性取出全年日程，并按天展开成标记，避免每个日期都查一次数据库。
fn collect_marks(
    events: Vec<Event>,
    year: i32,
) -> HashMap<NaiveDate, Vec<DayMark>> {
    let year_start =
        NaiveDate::from_ymd_opt(year, 1, 1).expect("invalid year");
    let year_end = NaiveDate::from_ymd_opt(year + 1, 1, 1)
        .expect("invalid year");
    let last_day = year_end - Duration::days(1);

    let mut marks: HashMap<NaiveDate, Vec<DayMark>> = HashMap::new();

    for event in events {
        if is_holiday(&event) {
            continue;
        }

        // 跨年日程只保留落在当前年份内的部分。
        let mut date = event.start_date.max(year_start);
        let end = event.end_date.min(last_day);

        while date <= end {
            let day = marks.entry(date).or_default();
            // 单元格很窄，每天最多画三个点就够了。
            if day.len() < 3 {
                day.push(DayMark { color: event.color });
            }
            date += Duration::days(1);
        }
    }

    marks
}

fn day_cell(
    date: NaiveDate,
    today: NaiveDate,
    selected: NaiveDate,
    marks: &[DayMark],
    cx: &Context<State>,
) -> impl IntoElement {
    let theme = cx.theme();
    let is_today = date == today;
    let is_selected = date == selected;
    let is_weekend =
        matches!(date.weekday().num_days_from_monday(), 5 | 6);

    let mut color = if is_weekend {
        theme.secondary_foreground
    } else {
        theme.foreground
    };
    if is_today {
        color = theme.primary_foreground;
    }

    let mut cell = h_flex()
        .h(px(20.))
        .w_full()
        .items_center()
        .justify_center()
        .gap(px(2.))
        .rounded(px(5.))
        .hover(|this| this.bg(theme.muted))
        .on_mouse_up(
            gpui_kit::MouseButton::Left,
            cx.listener(move |state, _, _, cx| {
                state.change_select_date(date);
                cx.notify();
            }),
        );

    if is_today {
        cell = cell.bg(theme.primary);
    } else if is_selected {
        cell = cell.bg(theme.primary.opacity(0.16));
    }

    let mut number = div()
        .text_xs()
        .font_medium()
        .text_color(color)
        .child(date.day().to_string());
    if is_selected && !is_today {
        number = number.text_color(theme.primary);
    }

    cell = cell.child(number);

    for mark in marks.iter().take(3) {
        cell = cell.child(
            div()
                .w(px(3.))
                .h(px(3.))
                .flex_shrink_0()
                .rounded_full()
                .bg(rgb(mark.color)),
        );
    }

    cell
}

fn month_card(
    year: i32,
    month: u32,
    today: NaiveDate,
    selected: NaiveDate,
    marks: &HashMap<NaiveDate, Vec<DayMark>>,
    cx: &Context<State>,
) -> impl IntoElement {
    let theme = cx.theme();
    let first = NaiveDate::from_ymd_opt(year, month, 1)
        .expect("invalid month");
    let offset = first.weekday().num_days_from_monday() as i64;
    let start = first - Duration::days(offset);

    let is_current_month =
        today.year() == year && today.month() == month;
    let is_selected_month =
        selected.year() == year && selected.month() == month;

    let mut head = h_flex()
        .w_full()
        .items_center()
        .justify_between()
        .px(px(2.))
        .child(
            div()
                .text_sm()
                .font_semibold()
                .text_color(if is_current_month {
                    theme.primary
                } else {
                    theme.foreground
                })
                .child(format!("{}月", month)),
        );

    if is_current_month {
        head = head.child(
            div().text_xs().text_color(theme.primary).child("本月"),
        );
    } else if is_selected_month {
        head = head.child(
            div()
                .w(px(5.))
                .h(px(5.))
                .rounded_full()
                .bg(theme.primary),
        );
    }

    let mut weekdays = div().grid().grid_cols(7).gap(px(1.));
    for (i, weekday) in WEEKDAYS.iter().enumerate() {
        weekdays = weekdays.child(
            div()
                .text_xs()
                .text_center()
                .text_color(if i >= 5 {
                    theme.muted_foreground
                } else {
                    theme.secondary_foreground
                })
                .child(*weekday),
        );
    }

    let mut grid = div().grid().grid_cols(7).gap(px(1.));
    // 固定 6 行，保证 12 个月卡片高度一致。
    for n in 0..42 {
        let date = start + Duration::days(n);
        if date.month() != month {
            grid = grid.child(div().h(px(20.)));
        } else {
            let empty = Vec::new();
            let day_marks = marks.get(&date).unwrap_or(&empty);
            grid = grid.child(day_cell(
                date, today, selected, day_marks, cx,
            ));
        }
    }

    v_flex()
        .h_full()
        .min_h(px(0.))
        .gap(px(3.))
        .p(px(6.))
        .rounded(px(10.))
        .border_1()
        .border_color(if is_current_month {
            theme.primary.opacity(0.5)
        } else {
            theme.border
        })
        .bg(if is_current_month {
            theme.primary.opacity(0.05)
        } else {
            theme.muted.opacity(0.3)
        })
        .overflow_hidden()
        .child(head)
        .child(weekdays)
        .child(grid)
}

/// 年视图：12 个月概览。点击某一天会选中该日期，
/// 再切换到月视图即可查看当日详情。
pub fn year_view(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    let theme = cx.theme();
    let year = state.select_date().year();
    let today = *state.today();
    let selected = *state.select_date();

    let marks = collect_marks(
        state.backend().get_events_between(
            NaiveDate::from_ymd_opt(year, 1, 1)
                .expect("invalid year"),
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
                .expect("invalid year"),
        ),
        year,
    );

    let mut months = div().grid().grid_cols(4).grid_rows(3).gap_2();
    for month in 1..=12 {
        months = months.child(month_card(
            year, month, today, selected, &marks, cx,
        ));
    }

    v_flex()
        .size_full()
        .p_3()
        .bg(theme.background)
        .child(months.flex_1().min_h(px(0.)).w_full())
}
