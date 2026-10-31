use gpui_kit::{
    Context, IntoElement, ParentElement, Styled, Window,
    base::{h_flex, v_flex},
    component::{
        WindowExt,
        button::{Button, ButtonVariants},
        date_picker::DatePicker,
        input::{Input, Textarea},
        switch::Switch,
    },
    div,
    prelude::FluentBuilder,
};

use crate::state::State;

pub fn new_event(
    state: &mut State,
    window: &mut Window,
    cx: &mut Context<State>,
) {
    let title_state = state.title_state_clone();
    let all_day = state.all_day_clone();
    let start_time_state = state.start_time_state_clone();
    let end_time_state = state.end_time_stat_clone();
    let description_state = state.description_state_clone();
    window.open_dialog(cx, move |dialog, w, app| {
        let all_day_clone = all_day.clone();
        let is_all_day = *all_day.read(app);
        dialog
            .title("新建日程")
            .child(
                v_flex()
                    .gap_2()
                    .child(Input::new(&title_state))
                    .child(
                        h_flex()
                            .justify_between()
                            .child("全天")
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
                    .child(DatePicker::new(&start_time_state))
                    .when(!is_all_day, |this| {
                        this.child(DatePicker::new(&end_time_state))
                    })
                    .child(Textarea::new(&description_state)),
            )
            .footer(
                h_flex()
                    .justify_between()
                    .child(
                        Button::new("取消").label("Cancel").on_click(
                            |_, window, cx| {
                                window.close_dialog(cx);
                            },
                        ),
                    )
                    .child(
                        Button::new("ok")
                            .primary()
                            .label("保存")
                            .on_click(|_, window, cx| {
                                window.close_dialog(cx);
                            }),
                    ),
            )
    });
}
