use chrono::{Datelike, Local, NaiveDate, Timelike};

use gpui_kit::{
    AnyElement, Context, Hsla, IntoElement, ParentElement, Styled,
    base::{StyledExt, h_flex, v_flex},
    component::{ActiveTheme, Theme, scroll::ScrollableElement},
    div, px, relative, rgb,
};

use crate::{
    backend::{Backend, Event},
    state::State,
};

use super::month_view::in_holiday;

/// 时间轴上一小时的高度。
const HOUR_HEIGHT: f32 = 56.;
/// 一天的总分钟数。
const DAY_MINUTES: i64 = 24 * 60;

const WEEKDAYS_FULL: [&str; 7] = [
    "星期日",
    "星期一",
    "星期二",
    "星期三",
    "星期四",
    "星期五",
    "星期六",
];

/// 时间轴上的一个日程，以及它在重叠组里占用的列。
struct TimedEvent {
    /// 在传入的日程列表中的下标。
    index: usize,
    start: i64,
    end: i64,
    col: usize,
    cols: usize,
}

/// 把日程换算成当天的起止分钟。全天事件和没有开始时间的事件
/// 返回 None，交给上方的“全天”栏展示。
///
/// 跨天日程会被裁剪到当天：从当天 0 点开始，或到当天 24 点结束。
fn time_range(event: &Event, date: NaiveDate) -> Option<(i64, i64)> {
    let start_time = event.start_time?;

    let start = if event.start_date < date {
        0
    } else {
        start_time.hour() as i64 * 60 + start_time.minute() as i64
    };

    let end = if event.end_date > date {
        DAY_MINUTES
    } else {
        event
            .end_time
            .map(|time| {
                time.hour() as i64 * 60 + time.minute() as i64
            })
            // 只有开始时间时默认持续一小时。
            .unwrap_or(start + 60)
    };

    // 极短的日程也要留出可点的区域。
    let start = start.min(DAY_MINUTES - 15).max(0);
    let end = end.max(start + 15).min(DAY_MINUTES);

    Some((start, end))
}

/// 计算重叠日程的分列：先按开始时间分组，组内贪心地放进
/// 最早空闲的列，同一组的日程平分宽度。
fn timed_layout(
    events: &[&Event],
    date: NaiveDate,
) -> Vec<TimedEvent> {
    let mut items: Vec<(i64, i64, usize)> = events
        .iter()
        .enumerate()
        .filter_map(|(index, event)| {
            let (start, end) = time_range(event, date)?;
            Some((start, end, index))
        })
        .collect();
    items.sort_by_key(|&(start, end, _)| (start, end));

    let mut layout = Vec::with_capacity(items.len());
    let mut group_start = 0;

    while group_start < items.len() {
        // 找出一组互相重叠的日程。
        let mut group_end = group_start;
        let mut max_end = items[group_start].1;
        while group_end + 1 < items.len()
            && items[group_end + 1].0 < max_end
        {
            group_end += 1;
            max_end = max_end.max(items[group_end].1);
        }

        let mut col_ends: Vec<i64> = Vec::new();
        let mut cols = Vec::new();
        for &(start, end, _) in &items[group_start..=group_end] {
            let col =
                match col_ends.iter().position(|&end| end <= start) {
                    Some(col) => col,
                    None => {
                        col_ends.push(0);
                        col_ends.len() - 1
                    }
                };
            col_ends[col] = end;
            cols.push(col);
        }

        let total_cols = col_ends.len();
        for (offset, &(start, end, index)) in
            items[group_start..=group_end].iter().enumerate()
        {
            layout.push(TimedEvent {
                index,
                start,
                end,
                col: cols[offset],
                cols: total_cols,
            });
        }

        group_start = group_end + 1;
    }

    layout
}

fn clock(minutes: i64) -> String {
    format!("{:02}:{:02}", minutes / 60, minutes % 60)
}

fn tag(text: String, color: Hsla) -> AnyElement {
    h_flex()
        .h(px(18.))
        .px(px(6.))
        .gap(px(4.))
        .rounded(px(4.))
        .bg(color.opacity(0.16))
        .text_xs()
        .text_color(color)
        .items_center()
        .child(text)
        .into_any_element()
}

fn header(
    date: NaiveDate,
    today: NaiveDate,
    count: usize,
    holiday: Option<(i64, i64, String)>,
    theme: &Theme,
) -> impl IntoElement {
    let mut tags: Vec<AnyElement> = Vec::new();

    if date == today {
        tags.push(
            div()
                .h(px(18.))
                .px(px(6.))
                .rounded(px(4.))
                .bg(theme.primary)
                .text_color(theme.primary_foreground)
                .text_xs()
                .flex()
                .items_center()
                .child("今天")
                .into_any_element(),
        );
    }

    if let Some((index, period, title)) = holiday {
        tags.push(tag(
            format!("{}假期 · 第 {} / {} 天", title, index, period),
            theme.danger,
        ));
    }

    h_flex()
        .w_full()
        .items_center()
        .gap_2()
        .p_3()
        .rounded(px(10.))
        .border_1()
        .border_color(theme.border)
        .bg(theme.muted.opacity(0.35))
        .child(
            div()
                .text_xl()
                .font_semibold()
                .text_color(theme.foreground)
                .child(format!("{}月{}日", date.month(), date.day())),
        )
        .child(
            div()
                .text_sm()
                .text_color(theme.secondary_foreground)
                .child(format!(
                    "{}年 · {}",
                    date.year(),
                    WEEKDAYS_FULL[date
                        .weekday()
                        .num_days_from_sunday()
                        as usize],
                )),
        )
        .child(h_flex().flex_wrap().gap_1().children(tags))
        .child(div().flex_grow_1())
        .child(
            div()
                .text_xs()
                .text_color(theme.muted_foreground)
                .child(format!("共 {} 项日程", count)),
        )
}

fn all_day_chip(event: &Event) -> impl IntoElement {
    let color = rgb(event.color);
    h_flex()
        .gap(px(4.))
        .px(px(6.))
        .py(px(2.))
        .rounded(px(5.))
        .bg(color.opacity(0.16))
        .child(
            div()
                .w(px(5.))
                .h(px(5.))
                .flex_shrink_0()
                .rounded_full()
                .bg(color),
        )
        .child(
            div()
                .text_xs()
                .text_color(color)
                .child(event.title.clone()),
        )
}

/// 全天事件（含节假日）栏：没有全天事件时不占空间。
fn all_day_row(events: &[&Event], theme: &Theme) -> impl IntoElement {
    h_flex()
        .w_full()
        .items_start()
        .gap_2()
        .px(px(8.))
        .py(px(6.))
        .rounded(px(8.))
        .border_1()
        .border_color(theme.border)
        .bg(theme.muted.opacity(0.25))
        .child(
            div()
                .w(px(44.))
                .flex_shrink_0()
                .pt(px(2.))
                .text_xs()
                .text_color(theme.muted_foreground)
                .child("全天"),
        )
        .child(
            h_flex()
                .flex_1()
                .min_w(px(0.))
                .flex_wrap()
                .gap_1()
                .children(
                    events.iter().map(|event| all_day_chip(event)),
                ),
        )
}

fn event_block(
    event: &Event,
    start: i64,
    end: i64,
    col: usize,
    cols: usize,
    theme: &Theme,
) -> impl IntoElement {
    let color = rgb(event.color);
    let top = start as f32 / 60. * HOUR_HEIGHT;
    let height = ((end - start) as f32 / 60. * HOUR_HEIGHT)
        .max(20.)
        .min(24. * HOUR_HEIGHT - top);

    let mut content = v_flex()
        .size_full()
        .gap(px(1.))
        .px(px(5.))
        .py(px(2.))
        .rounded(px(6.))
        .border_l_2()
        .border_color(color)
        .bg(color.opacity(0.18))
        .overflow_hidden()
        .child(
            div()
                .text_xs()
                .font_medium()
                .text_color(color)
                .truncate()
                .child(event.title.clone()),
        );

    // 太矮的日程只显示标题，避免文字被裁掉一半。
    if height >= 34. {
        content = content.child(
            div()
                .text_xs()
                .text_color(theme.secondary_foreground)
                .child(format!("{} - {}", clock(start), clock(end))),
        );
    }

    div()
        .absolute()
        .top(px(top))
        .left(relative(col as f32 / cols as f32))
        .w(relative(1. / cols as f32))
        .h(px(height))
        .px(px(1.))
        .child(content)
}

fn timeline(
    events: &[&Event],
    date: NaiveDate,
    theme: &Theme,
) -> impl IntoElement {
    let total_height = 24. * HOUR_HEIGHT;

    // 左侧时间刻度。
    let mut gutter =
        v_flex().w(px(52.)).h(px(total_height)).flex_shrink_0();
    for hour in 0..24 {
        gutter = gutter.child(
            div()
                .w_full()
                .h(px(HOUR_HEIGHT))
                .pr(px(8.))
                .flex()
                .justify_end()
                .text_xs()
                .text_color(theme.muted_foreground)
                .child(format!("{:02}:00", hour)),
        );
    }

    // 右侧网格与日程。
    let mut grid =
        div().relative().flex_1().min_w(px(0.)).h(px(total_height));

    for hour in 1..24 {
        grid = grid.child(
            div()
                .absolute()
                .top(px(hour as f32 * HOUR_HEIGHT))
                .left(px(0.))
                .w_full()
                .border_t_1()
                .border_color(theme.border.opacity(0.7)),
        );
    }

    for slot in timed_layout(events, date) {
        grid = grid.child(event_block(
            events[slot.index],
            slot.start,
            slot.end,
            slot.col,
            slot.cols,
            theme,
        ));
    }

    // 今天再加一条当前时间线。
    if date == Local::now().date_naive() {
        let now = Local::now();
        let top = (now.hour() as f32 * 60. + now.minute() as f32)
            / 60.
            * HOUR_HEIGHT;
        grid = grid
            .child(
                div()
                    .absolute()
                    .top(px(top))
                    .left(px(0.))
                    .w_full()
                    .h(px(1.))
                    .bg(theme.danger),
            )
            .child(
                div()
                    .absolute()
                    .top(px(top - 3.))
                    .left(px(-3.))
                    .w(px(7.))
                    .h(px(7.))
                    .rounded_full()
                    .bg(theme.danger),
            );
    }

    div()
        .w_full()
        .flex_1()
        .min_h(px(0.))
        .overflow_y_scrollbar()
        .child(
            h_flex()
                .w_full()
                .h(px(total_height))
                .items_start()
                .child(gutter)
                .child(grid),
        )
}

/// 日视图：左侧时间刻度 + 右侧时间轴，重叠日程自动分列。
pub fn day_view(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    let backend = cx.global::<Backend>();

    let theme = cx.theme();
    let date = *state.select_date();
    let today = Local::now().date_naive();
    let events = backend.get_event(date);

    // 有开始时间的进时间轴，其余（节假日、全天日程）进顶部栏。
    let mut all_day: Vec<&Event> = Vec::new();
    let mut timed: Vec<&Event> = Vec::new();
    for event in &events {
        if event.start_time.is_some() {
            timed.push(event);
        } else {
            all_day.push(event);
        }
    }
    all_day.sort_by(|a, b| {
        a.start_date
            .cmp(&b.start_date)
            .then_with(|| a.title.cmp(&b.title))
    });

    let mut content = v_flex()
        .size_full()
        .p_3()
        .gap_2()
        .bg(theme.background)
        .child(header(
            date,
            today,
            events.len(),
            in_holiday(state, backend),
            theme,
        ));

    if !all_day.is_empty() {
        content = content.child(all_day_row(&all_day, theme));
    }

    content.child(timeline(&timed, date, theme))
}
