use chrono::{Datelike, Duration, NaiveDate};

use gpui_kit::{
    Context, Hsla, InteractiveElement, IntoElement, ParentElement,
    Styled, Window,
    base::{StyledExt, h_flex, v_flex},
    component::{ActiveTheme, Theme},
    div, px, rgb,
};

use crate::{
    backend::{Backend, Cal, Event},
    state::State,
};

/// 星期表头，与参考网页一致，从周一开始。
const WEEKDAYS: [&str; 7] =
    ["周一", "周二", "周三", "周四", "周五", "周六", "周日"];
const WEEKDAYS_FULL: [&str; 7] = [
    "星期日",
    "星期一",
    "星期二",
    "星期三",
    "星期四",
    "星期五",
    "星期六",
];

/// 各分类统一取项目主题色，不使用参考网页里的配色。
fn cal_color(theme: &Theme, cal: Cal) -> Hsla {
    match cal {
        Cal::Rest => theme.danger,
        Cal::Work => theme.secondary_foreground,
        Cal::Leave => theme.success,
        Cal::None => gpui_kit::transparent_black(),
    }
}

fn badge(text: &'static str, color: Hsla) -> impl IntoElement {
    div()
        .h(px(16.))
        .px(px(5.))
        .rounded(px(4.))
        .bg(color.opacity(0.18))
        .text_color(color)
        .text_xs()
        .flex()
        .items_center()
        .child(text)
}

/// 月视图单元格里的日程条（占满宽度）。
fn chip(event: &Event, theme: &Theme) -> impl IntoElement {
    let color = cal_color(theme, event.cal);
    h_flex()
        .w_full()
        .gap(px(3.))
        .px(px(4.))
        .py(px(1.))
        .rounded(px(5.))
        .bg(color.opacity(0.16))
        .child(
            div()
                .w(px(4.))
                .h(px(4.))
                .rounded_full()
                .flex_shrink_0()
                .bg(color),
        )
        .child(
            div()
                .flex_1()
                .min_w(px(0.))
                .truncate()
                .text_xs()
                .text_color(color)
                .child(event.title.clone()),
        )
}

/// 详情面板里的标签（宽度自适应）。
fn tag(label: String, color: Hsla) -> impl IntoElement {
    h_flex()
        .gap_1()
        .px(px(8.))
        .py(px(3.))
        .rounded(px(6.))
        .bg(color.opacity(0.16))
        .child(
            div()
                .w(px(5.))
                .h(px(5.))
                .rounded_full()
                .flex_shrink_0()
                .bg(color),
        )
        .child(div().text_xs().text_color(color).child(label))
}

fn day_cell(
    state: &mut State,
    cx: &Context<State>,
    date: NaiveDate,
) -> impl IntoElement {
    let backend = cx.global::<Backend>();

    let is_weekend =
        matches!(date.weekday().num_days_from_monday(), 5 | 6);
    let is_outside = date.month() != state.select_date().month();
    let is_today = date.eq(state.today());
    let is_selected = date.eq(state.select_date());

    let mut bg = if is_outside {
        cx.theme().background
    } else {
        cx.theme().muted
    };

    let mut cell = v_flex()
        .gap(px(3.))
        .p(px(6.))
        .rounded(px(10.))
        .border_1()
        .border_color(if is_outside {
            cx.theme().background
        } else {
            cx.theme().border
        })
        .on_mouse_up(
            gpui_kit::MouseButton::Left,
            cx.listener(move |state, _, _, _| {
                state.change_select_date(date);
            }),
        )
        .overflow_hidden();

    if is_today {
        bg = cx.theme().primary.opacity(0.14);
    }
    if is_selected {
        bg = cx.theme().primary.opacity(0.14);
        cell = cell.border_color(cx.theme().primary);
    }
    cell = cell.bg(bg);

    // 标题行：日期数字 + 角标
    let num_color = if is_today {
        cx.theme().primary_foreground
    } else if is_outside {
        cx.theme().muted_foreground
    } else if is_weekend {
        cx.theme().secondary_foreground
    } else {
        cx.theme().foreground
    };

    let mut number = div()
        .min_w(px(24.))
        .h(px(24.))
        .px(px(6.))
        .rounded_full()
        .flex()
        .items_center()
        .justify_center()
        .text_sm()
        .font_semibold()
        .text_color(num_color)
        .child(date.day().to_string());
    if is_today {
        number = number.bg(cx.theme().primary);
    }

    let events = backend.get_event(date);

    let mut top = h_flex()
        .w_full()
        .justify_between()
        .items_center()
        .child(number);
    if events.iter().any(|event| event.cal == Cal::Rest) {
        top = top.child(badge("休", cx.theme().danger));
    } else if events.iter().any(|s| s.cal == Cal::Work) {
        top = top.child(badge("班", cx.theme().secondary_foreground));
    } else if events.iter().any(|s| s.cal == Cal::Leave) {
        top = top.child(badge("假", cx.theme().success));
    }
    cell = cell.child(top);

    // 日程条：最多三条，多余收起为 +N
    let mut chips = v_flex().w_full().gap(px(3.)).min_w(px(0.));
    for event in events.iter().take(3) {
        chips = chips.child(chip(&event, cx.theme()));
    }
    if events.len() > 3 {
        chips = chips.child(
            div()
                .pl(px(2.))
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(format!("+{} 项", events.len() - 3)),
        );
    }
    cell = cell.child(chips);

    cell
}

pub fn month_view(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    let anchor = state.select_date();

    let first =
        NaiveDate::from_ymd_opt(anchor.year(), anchor.month(), 1)
            .unwrap();
    let offset = first.weekday().num_days_from_monday() as i64;
    //这个月的周一
    let start = first - Duration::days(offset);

    let mut head = div().grid().grid_cols(7).gap_1();
    for (i, wd) in WEEKDAYS.iter().enumerate() {
        let color = if i >= 5 {
            cx.theme().muted_foreground
        } else {
            cx.theme().secondary_foreground
        };
        head = head.child(
            div()
                .py(px(2.))
                .text_sm()
                .font_medium()
                .text_center()
                .text_color(color)
                .child(*wd),
        );
    }

    let mut grid = div().grid().grid_cols(7).grid_rows(6).gap_1();
    for n in 0..42 {
        let date = start + Duration::days(n);
        grid = grid.child(day_cell(state, cx, date));
    }

    v_flex()
        .size_full()
        .gap_2()
        .p_3()
        .bg(cx.theme().background)
        .child(head)
        .child(grid.flex_1().min_h(px(0.)).w_full())
}

/// 参考网页中 id="detail" 的右侧日期详情面板。
fn detail(
    state: &mut State,
    cx: &Context<State>,
    _: &mut Window,
) -> impl IntoElement {
    let backend = cx.global::<Backend>();

    let theme = cx.theme();
    let is_today = state.is_today();
    let date = *state.select_date();

    // ---------- 头部：日期、星期与假期标签 ----------
    let mut tags: Vec<gpui_kit::AnyElement> = Vec::new();
    if is_today {
        tags.push(
            div()
                .h(px(16.))
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

    let in_holiday = in_holiday(state, backend);
    if let Some((index, period, title)) = in_holiday {
        tags.push(
            tag(
                format!(
                    "{}假期 · 第 {} / {} 天",
                    title, index, period
                ),
                theme.danger,
            )
            .into_any_element(),
        );
    }
    if tags.is_empty() {
        tags.push(
            div()
                .text_sm()
                .text_color(theme.secondary_foreground)
                .child(
                    if matches!(
                        date.weekday().num_days_from_monday(),
                        5 | 6
                    ) {
                        "周末休息日"
                    } else {
                        "工作日"
                    },
                )
                .into_any_element(),
        );
    }

    let head = v_flex()
        .gap_2()
        .p_4()
        .border_b_1()
        .border_color(theme.border)
        .child(
            v_flex()
                .gap_0()
                .child(
                    div()
                        .text_2xl()
                        .font_semibold()
                        .text_color(theme.foreground)
                        .child(format!(
                            "{}月{}日",
                            date.month(),
                            date.day()
                        )),
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
                ),
        )
        .child(h_flex().flex_wrap().gap_1p5().children(tags));

    let events = backend.get_event(date);

    let mut body = v_flex()
        .flex_1()
        .min_h(px(0.))
        .gap_3()
        .p_4()
        .overflow_hidden()
        .child(
            div()
                .text_xs()
                .font_semibold()
                .text_color(theme.muted_foreground)
                .child(format!("当日日程 · {} 项", events.len())),
        );

    if events.is_empty() {
        body = body.child(
            v_flex()
                .gap_2()
                .p_4()
                .rounded(px(10.))
                .border_1()
                .border_color(theme.border)
                .bg(theme.muted.opacity(0.4))
                .child(
                    div()
                        .text_sm()
                        .font_semibold()
                        .text_color(theme.foreground)
                        .child("这一天还没有日程"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.secondary_foreground)
                        .child("选中月视图中的其他日期查看详情，或用下方按钮为这一天添加日程。"),
                ),
        );
    } else {
        let mut list = v_flex().gap_2().w_full();
        for event in events {
            let color = rgb(event.color);
            //todo  这里要实可能有只有几个小时的日程
            // let time = if seed.start.is_empty() {
            //     "全天".to_string()
            // } else {
            //     format!("{}-{}", seed.start, seed.end)
            // };
            let mut item = h_flex()
                .w_full()
                .items_start()
                .gap_3()
                .p_3()
                .rounded(px(10.))
                .border_1()
                .border_color(theme.border)
                .child(
                    div()
                        .w(px(10.))
                        .h(px(10.))
                        .mt(px(4.))
                        .rounded(px(3.))
                        .flex_shrink_0()
                        .bg(color),
                )
                .child(
                    v_flex()
                        .flex_1()
                        .min_w(px(0.))
                        .gap(px(2.))
                        .child(
                            div()
                                .text_sm()
                                .font_semibold()
                                .text_color(theme.foreground)
                                .child(event.title),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(
                                    theme.secondary_foreground,
                                )
                                .child(format!(
                                    "{} · {}",
                                    "全天", event.category_name
                                )),
                        ),
                );
            if let Some(desc) = event.description {
                item = item.child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child(desc),
                );
            }
            list = list.child(item);
        }
        body = body.child(list);
    }

    v_flex()
        .h_full()
        .w(px(220.))
        .flex_shrink_0()
        .border_l_1()
        .border_color(theme.border)
        .child(head)
        .child(body)
    // .child(foot)
}

pub fn main_content(
    state: &mut State,
    cx: &Context<State>,
    window: &mut Window,
) -> impl IntoElement {
    div()
        .flex()
        .size_full()
        .bg(cx.theme().background)
        .child(month_view(state, cx))
        .child(detail(state, cx, window))
}

pub fn in_holiday(
    state: &State,
    backend: &Backend,
) -> Option<(i64, i64, String)> {
    let date = *state.select_date();
    let event = backend.get_event_by_date(1, date);

    event.map(|event| {
        let start_date = event.start_date;

        let index = (date - start_date).num_days() + 1;

        let period =
            (event.end_date - event.start_date).num_days() + 1;

        (index, period, event.title)
    })
}

#[cfg(test)]
mod test {
    use chrono::{Local, NaiveDate};

    #[test]
    fn test_add_day() {
        let now = Local::now().date_naive();
        let t = NaiveDate::from_ymd_opt(2026, 10, 20).unwrap();
        let day = t - now;
        println!("day:{},hours:{}", day.num_days(), day.num_hours())
    }
}
