use core::time;

use gpui_kit::{
    App, AppContext, Context, Entity, IntoElement, ParentElement,
    Styled, Window,
    base::{
        ElementExt, h_flex,
        input::{InputState, TextareaState},
        v_flex,
    },
    component::{
        Sizable, Size, WindowExt,
        button::{Button, ButtonVariants},
        date_picker::{DatePicker, DatePickerState},
        input::{Input, Textarea},
        select::{Select, SelectState},
        switch::Switch,
    },
    div,
    prelude::FluentBuilder,
    px,
};

use crate::{
    backend::Backend, ext::DatePickerAtBaseRem, state::State,
};

pub fn new_event(
    state: &mut State,
    window: &mut Window,
    cx: &mut Context<State>,
) {
    let calendar_state = state.calendar_state_clone();
    let title_state = state.title_state_clone();
    let all_day = state.all_day_clone();
    let start_time_state = state.start_time_state_clone();
    let end_time_state = state.end_time_stat_clone();
    let description_state = state.description_state_clone();

    window.open_dialog(cx, move |dialog, window, app| {
        let all_day_clone = all_day.clone();
        let is_all_day = *all_day.read(app);

        let calendar = calendar_state.clone();
        let title = title_state.clone();
        let start_time = start_time_state.clone();
        let end_time = end_time_state.clone();
        let descc_state = description_state.clone();
        dialog
            .title("新建日程")
            .child(
                v_flex()
                    .mt_4()
                    .gap_4()
                    .child(
                        h_flex()
                            .justify_between()
                            .child("日历")
                            .child(Select::new(&calendar_state)),
                    )
                    .child(Input::new(&title_state))
                    .child(
                        h_flex()
                            .justify_between()
                            .child("全天事件")
                            .child(
                                Switch::new("all_day_switch")
                                    .checked(is_all_day)
                                    .on_change(
                                        move |&checkedd, _, app| {
                                            all_day_clone
                                                .write(app, checkedd);
                                        },
                                    ),
                            ),
                    )
                    .child(time_select(
                        window,
                        &start_time_state,
                        "开始时间",
                    ))
                    .when(!is_all_day, |this| {
                        this.child(time_select(
                            window,
                            &end_time_state,
                            "结束时间",
                        ))
                    })
                    .child(Textarea::new(&description_state)),
            )
            .footer(
                h_flex()
                    .gap_2()
                    .justify_end()
                    .child(
                        Button::new("cancel").label("取消").on_click(
                            |_, window, cx| {
                                window.close_dialog(cx);
                            },
                        ),
                    )
                    .child(
                        Button::new("ok")
                            .primary()
                            .label("保存")
                            .on_click(move |e, window, app| {
                                save_event(
                                    &calendar,
                                    &title,
                                    is_all_day,
                                    &start_time,
                                    &end_time,
                                    &descc_state,
                                    app,
                                );
                                window.close_dialog(app);
                            }),
                    ),
            )
    });
}

//hack方式，DatePicker的Calendar用的是写死的宽度
//改变字体后，UI会溢出。这里重新改回默认字体。
fn time_select(
    window: &mut Window,
    entity: &Entity<DatePickerState>,
    placeholder: &str,
) -> impl IntoElement {
    let previous_rem_size = window.rem_size();
    window.set_rem_size(px(16.));

    div()
        .size_full()
        .on_prepaint(move |_, window, _| {
            window.set_rem_size(previous_rem_size)
        })
        .child(
            DatePicker::new(entity)
                .appearance(true)
                .placeholder(placeholder),
        )
}

fn save_event(
    calendar: &Entity<SelectState<Vec<String>>>,
    title: &Entity<InputState>,
    all_day: bool,
    start_time: &Entity<DatePickerState>,
    end_time: &Entity<DatePickerState>,
    desc: &Entity<TextareaState>,
    app: &mut App,
) {
    let db = app.global::<Backend>();
    let c = calendar.read(app);
    let calendar_index = c.selected_index(app).unwrap().row + 1;
    let calendar = c.selected_value().unwrap().clone();
    let title = title.read(app).content().text().to_string();

    let start_time = start_time.read(app).date_time();
    let description = desc.read(app).content().text().to_string();
    println!("desc:{}", description);
    db.save_event(
        calendar_index as u32,
        calendar,
        title,
        all_day,
        start_time,
        if all_day {
            start_time
        } else {
            end_time.read(app).date_time()
        },
        description,
    );
}
