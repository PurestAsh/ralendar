use chrono::Local;

use gpui_kit::{
    App, AppContext, Context, Div, Entity, Hsla, IntoElement,
    ParentElement, Render, Styled, Window,
    assets::IconName,
    base::{StyledExt as _, h_flex, v_flex},
    component::{
        ActiveTheme, WindowExt,
        button::{Button, ButtonVariants},
        input::{Input, InputState},
        switch::Switch,
    },
    div, px,
};

/// 打开“新建日程”对话框。
///
/// 目前只负责 UI，保存时直接关闭对话框，还没有接后端。
pub fn new_event(window: &mut Window, app: &mut App) {
    let form = app.new(|cx| NewEventForm::new(window, cx));

    window.open_dialog(app, move |dialog, _window, _app| {
        dialog
            .title("新建日程")
            .w(px(560.))
            .child(form.clone())
            .footer(
                h_flex()
                    .w_full()
                    .justify_between()
                    .gap_2()
                    .child(Button::new("cancel").label("取消").on_click(
                        |_, window, cx| {
                            window.close_dialog(cx);
                        },
                    ))
                    .child(
                        Button::new("save")
                            .primary()
                            .label("保存")
                            .on_click(|_, window, cx| {
                                window.close_dialog(cx);
                            }),
                    ),
            )
    })
}

struct NewEventForm {
    title: Entity<InputState>,
    start: Entity<InputState>,
    end: Entity<InputState>,
    location: Entity<InputState>,
    notes: Entity<InputState>,
    invitees: Entity<InputState>,
    all_day: bool,
}

impl NewEventForm {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let start_value = format!("{date} 10:00");
        let end_value = format!("{date} 11:00");

        Self {
            title: cx.new(|cx| {
                InputState::new(window, cx).placeholder("标题")
            }),
            start: cx.new(|cx| {
                InputState::new(window, cx).default_value(start_value)
            }),
            end: cx
                .new(|cx| InputState::new(window, cx).default_value(end_value)),
            location: cx.new(|cx| {
                InputState::new(window, cx).placeholder("地点")
            }),
            notes: cx.new(|cx| {
                InputState::new(window, cx).placeholder("备注")
            }),
            invitees: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("参与人（邮箱，逗号分隔）")
            }),
            all_day: false,
        }
    }
}

impl Render for NewEventForm {
    fn render(
        &mut self,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let border = theme.border;
        let foreground = theme.foreground;
        let muted = theme.muted_foreground;

        v_flex()
            .w_full()
            .rounded(px(10.))
            .border_1()
            .border_color(border)
            .overflow_hidden()
            .child(calendar_row(border, foreground, muted))
            .child(input_row("edit-title", &self.title, border))
            .child(
                row(border)
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .text_color(foreground)
                            .child("全天"),
                    )
                    .child(
                        Switch::new("all-day")
                            .checked(self.all_day)
                            .on_click(cx.listener(
                                |this, checked: &bool, _, cx| {
                                    this.all_day = *checked;
                                    cx.notify();
                                },
                            )),
                    ),
            )
            .child(time_row(
                "edit-start",
                "开始（YYYY-MM-DD HH:MM）",
                &self.start,
                border,
                muted,
            ))
            .child(time_row(
                "edit-end",
                "结束（YYYY-MM-DD HH:MM）",
                &self.end,
                border,
                muted,
            ))
            .child(select_row(
                "repeat-select",
                "重复",
                "不重复",
                border,
                foreground,
            ))
            .child(select_row(
                "alert-select",
                "提醒",
                "不提醒",
                border,
                foreground,
            ))
            .child(input_row("edit-location", &self.location, border))
            .child(input_row("edit-notes", &self.notes, border))
            .child(input_row("edit-invitees", &self.invitees, border))
    }
}

fn row(border: Hsla) -> Div {
    h_flex()
        .w_full()
        .items_center()
        .gap_3()
        .px(px(16.))
        .py(px(12.))
        .border_b_1()
        .border_color(border)
}

fn pencil_button(
    id: &'static str,
    input: &Entity<InputState>,
) -> Button {
    let input = input.clone();
    Button::new(id)
        .ghost()
        .compact()
        .icon(IconName::Pencil)
        .tooltip("编辑")
        .on_click(move |_, window, cx| {
            input.update(cx, |state, cx| state.focus(window, cx));
        })
}

fn calendar_row(
    border: Hsla,
    foreground: Hsla,
    muted: Hsla,
) -> impl IntoElement {
    row(border)
        .justify_between()
        .child(
            v_flex()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .font_medium()
                        .text_color(foreground)
                        .child("日历"),
                )
                .child(div().text_xs().text_color(muted).child("本地")),
        )
        .child(
            Button::new("calendar-select")
                .outline()
                .label("本地")
                .dropdown_caret(true),
        )
}

fn input_row(
    id: &'static str,
    input: &Entity<InputState>,
    border: Hsla,
) -> impl IntoElement {
    row(border)
        .justify_between()
        .child(
            div()
                .flex_1()
                .min_w(px(0.))
                .child(Input::new(input).appearance(false)),
        )
        .child(pencil_button(id, input))
}

fn time_row(
    id: &'static str,
    label: &'static str,
    input: &Entity<InputState>,
    border: Hsla,
    muted: Hsla,
) -> impl IntoElement {
    row(border)
        .justify_between()
        .child(
            v_flex()
                .flex_1()
                .min_w(px(0.))
                .gap_1()
                .child(div().text_xs().text_color(muted).child(label))
                .child(Input::new(input).appearance(false)),
        )
        .child(pencil_button(id, input))
}

fn select_row(
    id: &'static str,
    label: &'static str,
    value: &'static str,
    border: Hsla,
    foreground: Hsla,
) -> impl IntoElement {
    row(border)
        .justify_between()
        .child(
            div()
                .text_sm()
                .text_color(foreground)
                .child(label),
        )
        .child(
            Button::new(id)
                .ghost()
                .label(value)
                .dropdown_caret(true),
        )
}
