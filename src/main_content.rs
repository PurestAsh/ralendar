use chrono::{Datelike, Duration, Local, NaiveDate};

use gpui_kit::{
    Context, Hsla, IntoElement, ParentElement, Styled,
    base::{StyledExt, h_flex, v_flex},
    component::{
        ActiveTheme, IconName, Theme,
        button::{Button, ButtonVariants},
    },
    div, px,
};

use crate::state::State;

/// 参考网页里的日历分类：我的日程 / 中国节假日 / 国际节假日 / 调休。
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cal {
    Mine,
    Cn,
    Intl,
    Work,
    Leave,
}

struct Seed {
    day: u32,
    title: &'static str,
    cal: Cal,
    start: &'static str,
    end: &'static str,
    note: &'static str,
}

/// 参考网页中 2026 年 10 月的示例数据。
const SEED: &[Seed] = &[
    Seed {
        day: 1,
        title: "国庆节",
        cal: Cal::Cn,
        start: "",
        end: "",
        note: "",
    },
    Seed {
        day: 2,
        title: "国庆节",
        cal: Cal::Cn,
        start: "",
        end: "",
        note: "",
    },
    Seed {
        day: 3,
        title: "国庆节",
        cal: Cal::Cn,
        start: "",
        end: "",
        note: "",
    },
    Seed {
        day: 4,
        title: "国庆节",
        cal: Cal::Cn,
        start: "",
        end: "",
        note: "",
    },
    Seed {
        day: 5,
        title: "国庆节",
        cal: Cal::Cn,
        start: "",
        end: "",
        note: "",
    },
    Seed {
        day: 6,
        title: "国庆节",
        cal: Cal::Cn,
        start: "",
        end: "",
        note: "",
    },
    Seed {
        day: 7,
        title: "国庆节",
        cal: Cal::Cn,
        start: "",
        end: "",
        note: "",
    },
    Seed {
        day: 8,
        title: "国庆节",
        cal: Cal::Cn,
        start: "",
        end: "",
        note: "",
    },
    Seed {
        day: 9,
        title: "年假",
        cal: Cal::Leave,
        start: "",
        end: "",
        note: "",
    },
    Seed {
        day: 10,
        title: "调休上班",
        cal: Cal::Work,
        start: "",
        end: "",
        note: "",
    },
    Seed {
        day: 2,
        title: "家庭聚餐",
        cal: Cal::Mine,
        start: "12:00",
        end: "14:00",
        note: "外婆家",
    },
    Seed {
        day: 3,
        title: "假期书单打卡",
        cal: Cal::Mine,
        start: "21:00",
        end: "22:00",
        note: "",
    },
    Seed {
        day: 5,
        title: "项目周报整理",
        cal: Cal::Mine,
        start: "20:00",
        end: "21:00",
        note: "",
    },
    Seed {
        day: 7,
        title: "返程高铁 G102",
        cal: Cal::Mine,
        start: "15:30",
        end: "19:30",
        note: "二等座 08 车 12F",
    },
    Seed {
        day: 10,
        title: "述职准备",
        cal: Cal::Mine,
        start: "09:30",
        end: "10:30",
        note: "调休上班日",
    },
    Seed {
        day: 12,
        title: "季度复盘会",
        cal: Cal::Mine,
        start: "10:00",
        end: "11:30",
        note: "提前同步材料",
    },
    Seed {
        day: 15,
        title: "体检",
        cal: Cal::Mine,
        start: "08:30",
        end: "10:30",
        note: "空腹前往",
    },
    Seed {
        day: 18,
        title: "读书会",
        cal: Cal::Mine,
        start: "14:00",
        end: "16:00",
        note: "",
    },
    Seed {
        day: 21,
        title: "版本发布窗口",
        cal: Cal::Mine,
        start: "20:00",
        end: "22:00",
        note: "值守",
    },
    Seed {
        day: 24,
        title: "城市徒步",
        cal: Cal::Mine,
        start: "09:00",
        end: "12:00",
        note: "集合点：北门",
    },
    Seed {
        day: 26,
        title: "牙科复诊",
        cal: Cal::Mine,
        start: "18:00",
        end: "19:00",
        note: "",
    },
    Seed {
        day: 28,
        title: "团队分享会",
        cal: Cal::Mine,
        start: "16:00",
        end: "17:00",
        note: "分享日历工具",
    },
    Seed {
        day: 30,
        title: "月度总结",
        cal: Cal::Mine,
        start: "17:00",
        end: "18:00",
        note: "",
    },
    Seed {
        day: 31,
        title: "万圣夜",
        cal: Cal::Intl,
        start: "",
        end: "",
        note: "",
    },
];

/// 2026 年国庆假期（仅示例数据使用）。
const CN_HOLIDAY: (&str, u32, u32) = ("国庆节", 1, 8);

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
        Cal::Mine => theme.primary,
        Cal::Cn => theme.danger,
        Cal::Intl => theme.warning,
        Cal::Work => theme.secondary_foreground,
        Cal::Leave => theme.success,
    }
}

fn cal_name(cal: Cal) -> &'static str {
    match cal {
        Cal::Mine | Cal::Leave => "我的日程",
        Cal::Cn => "中国节假日",
        Cal::Intl => "国际节假日",
        Cal::Work => "调休上班日",
    }
}

fn visible(state: &State, cal: Cal) -> bool {
    match cal {
        Cal::Mine => state.my_schedule(),
        Cal::Cn | Cal::Work | Cal::Leave => state.cn_schedule(),
        Cal::Intl => state.en_schedule(),
    }
}

fn seeds_on(state: &State, date: NaiveDate) -> Vec<&'static Seed> {
    // 示例数据对应 2026 年 10 月，换月后不显示。
    if date.year() != 2026 || date.month() != 10 {
        return Vec::new();
    }
    SEED.iter()
        .filter(|s| s.day == date.day() && visible(state, s.cal))
        .collect()
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
fn chip(
    title: &'static str,
    cal: Cal,
    theme: &Theme,
) -> impl IntoElement {
    let color = cal_color(theme, cal);
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
                .child(title),
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
    theme: &Theme,
    date: NaiveDate,
    month: u32,
    today: NaiveDate,
    selected: NaiveDate,
    state: &State,
) -> impl IntoElement {
    let is_weekend =
        matches!(date.weekday().num_days_from_monday(), 5 | 6);
    let is_outside = date.month() != month;
    let is_today = date == today;
    let is_selected = date == selected;

    let mut bg = if is_outside {
        theme.background
    } else {
        theme.muted
    };

    let mut cell = v_flex()
        .gap(px(3.))
        .p(px(6.))
        .rounded(px(10.))
        .border_1()
        .border_color(if is_outside {
            theme.background
        } else {
            theme.border
        })
        .overflow_hidden();

    if is_today {
        bg = theme.primary.opacity(0.14);
        cell = cell.border_color(theme.primary);
    } else if is_selected {
        cell = cell.border_color(theme.primary);
    }
    cell = cell.bg(bg);

    // 标题行：日期数字 + 角标
    let num_color = if is_today {
        theme.primary_foreground
    } else if is_outside {
        theme.muted_foreground
    } else if is_weekend {
        theme.secondary_foreground
    } else {
        theme.foreground
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
        number = number.bg(theme.primary);
    }

    let events = seeds_on(state, date);

    let mut top = h_flex()
        .w_full()
        .justify_between()
        .items_center()
        .child(number);
    if events.iter().any(|s| s.cal == Cal::Cn) {
        top = top.child(badge("休", theme.danger));
    } else if events.iter().any(|s| s.cal == Cal::Work) {
        top = top.child(badge("班", theme.secondary_foreground));
    } else if events.iter().any(|s| s.cal == Cal::Leave) {
        top = top.child(badge("假", theme.success));
    }
    cell = cell.child(top);

    // 日程条：最多三条，多余收起为 +N
    let mut chips = v_flex().w_full().gap(px(3.)).min_w(px(0.));
    for seed in events.iter().take(3) {
        chips = chips.child(chip(seed.title, seed.cal, theme));
    }
    if events.len() > 3 {
        chips = chips.child(
            div()
                .pl(px(2.))
                .text_xs()
                .text_color(theme.muted_foreground)
                .child(format!("+{} 项", events.len() - 3)),
        );
    }
    cell = cell.child(chips);

    cell
}

fn month_view(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    let theme = cx.theme();
    let today = Local::now().date_naive();
    let anchor = state.select_date();

    let first =
        NaiveDate::from_ymd_opt(anchor.year(), anchor.month(), 1)
            .unwrap();
    let offset = first.weekday().num_days_from_monday() as i64;
    let start = first - Duration::days(offset);

    let mut head = div().grid().grid_cols(7).gap_1();
    for (i, wd) in WEEKDAYS.iter().enumerate() {
        let color = if i >= 5 {
            theme.muted_foreground
        } else {
            theme.secondary_foreground
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
        grid = grid.child(day_cell(
            theme,
            date,
            anchor.month(),
            today,
            anchor,
            state,
        ));
    }

    v_flex()
        .size_full()
        .gap_2()
        .p_3()
        .bg(theme.background)
        .child(head)
        .child(grid.flex_1().min_h(px(0.)).w_full())
}

/// 参考网页中 id="detail" 的右侧日期详情面板。
fn detail(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    let theme = cx.theme();
    let today = Local::now().date_naive();
    let date = state.select_date();
    let is_today = date == today;

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

    let (_, holiday_start, holiday_end) = CN_HOLIDAY;
    let in_holiday = date.year() == 2026
        && date.month() == 10
        && (holiday_start..=holiday_end).contains(&date.day());
    if in_holiday {
        let index = date.day() - holiday_start + 1;
        tags.push(
            tag(
                format!(
                    "{}假期 · 第 {} / {} 天",
                    CN_HOLIDAY.0,
                    index,
                    holiday_end - holiday_start + 1
                ),
                theme.danger,
            )
            .into_any_element(),
        );
    }
    if seeds_on(state, date).iter().any(|s| s.cal == Cal::Work) {
        tags.push(
            tag("调休上班日".to_string(), theme.secondary_foreground)
                .into_any_element(),
        );
    }
    if seeds_on(state, date).iter().any(|s| s.cal == Cal::Leave) {
        tags.push(
            tag("年假 · 个人请假".to_string(), theme.success)
                .into_any_element(),
        );
    }
    if let Some(seed) =
        seeds_on(state, date).iter().find(|s| s.cal == Cal::Intl)
    {
        tags.push(
            tag(seed.title.to_string(), theme.warning)
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

    // ---------- 主体：当日日程列表 ----------
    let events: Vec<&Seed> = seeds_on(state, date)
        .into_iter()
        .filter(|s| s.cal == Cal::Mine)
        .collect();

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
        for seed in events {
            let color = cal_color(theme, seed.cal);
            let time = if seed.start.is_empty() {
                "全天".to_string()
            } else {
                format!("{}–{}", seed.start, seed.end)
            };
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
                                .child(seed.title),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(
                                    theme.secondary_foreground,
                                )
                                .child(format!(
                                    "{} · {}",
                                    time,
                                    cal_name(seed.cal)
                                )),
                        ),
                );
            if !seed.note.is_empty() {
                item = item.child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child(seed.note),
                );
            }
            list = list.child(item);
        }
        body = body.child(list);
    }

    // ---------- 底部：为这一天添加日程 ----------
    let foot =
        div().p_3().border_t_1().border_color(theme.border).child(
            Button::new("add-here")
                .primary()
                .w_full()
                .icon(IconName::Plus)
                .child("在这一天添加日程"),
        );

    v_flex()
        .h_full()
        .w(px(220.))
        .flex_shrink_0()
        .border_l_1()
        .border_color(theme.border)
        .child(head)
        .child(body)
        .child(foot)
}

pub fn main_content(
    state: &mut State,
    cx: &Context<State>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .size_full()
        .bg(cx.theme().background)
        .child(month_view(state, cx))
        .child(detail(state, cx))
}
