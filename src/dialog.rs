use gpui_kit::{
    App, AppContext, Context, Entity, IntoElement, ParentElement,
    Render, Styled, Subscription, Window,
    base::{
        ColorPickerState, ElementExt, IndexPath, TimePrecision,
        h_flex,
        input::{InputState, TextareaState},
        v_flex,
    },
    component::{
        Sizable, WindowExt,
        button::{Button, ButtonVariants},
        color_picker::{ColorPicker, ColorSelect},
        date_picker::{DatePicker, DatePickerState},
        form::field,
        input::{Input, Textarea},
        select::{Select, SelectEvent, SelectState},
        switch::Switch,
    },
    div,
    prelude::FluentBuilder,
    px, rgb,
};

use crate::backend::{Backend, Category};

pub fn new_event(window: &mut Window, cx: &mut App) {
    let new_event_panel = cx.new(|cx| NewEventPanel::new(cx, window));
    window.open_dialog(cx, move |dialog, _, _| {
        let panel_clone = new_event_panel.clone();
        dialog
            .title("新建日程")
            .child(new_event_panel.clone())
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
                            .on_click(move |_, window, cx| {
                                panel_clone.read(cx).save(cx);

                                window.close_dialog(cx);
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
    // window.set_rem_size(px(16.));

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

/// 新建日程表单UI
struct NewEventPanel {
    //属于哪个日历
    calendar: Entity<SelectState<Vec<String>>>,
    //标题
    title: Entity<InputState>,
    //是否是全天事件
    all_day: bool,
    //开始时间
    start_time: Entity<DatePickerState>,
    //结束时间
    end_time: Entity<DatePickerState>,
    //颜色数据
    color_state: Entity<ColorPickerState>,
    //详细描述
    desc: Entity<TextareaState>,
    //日历选择变化的订阅
    _calendar_subscription: Subscription,
}

impl NewEventPanel {
    fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let categeries = cx.global::<Backend>();
        let cats = categeries.get_categories();
        let cats_name = cats
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<String>>();
        let default_color = cats.get(0).unwrap().color;
        let calendar = cx.new(|cx| {
            SelectState::new(
                cats_name,
                Some(IndexPath::default()), // Select first item
                window,
                cx,
            )
        });

        let color_state = cx.new(|cx| {
            ColorPickerState::new(window, cx)
                .default_value(rgb(default_color))
        });

        //日历切换时，把颜色同步为所选日历（分类）的颜色
        let _calendar_subscription = cx.subscribe_in(
            &calendar,
            window,
            move |this,
                  _,
                  event: &SelectEvent<Vec<String>>,
                  window,
                  cx| {
                let SelectEvent::Confirm(Some(name)) = event else {
                    return;
                };
                let Some(cat) = cats.iter().find(|c| &c.name == name)
                else {
                    return;
                };
                this.color_state.update(cx, |state, cx| {
                    state.set_value(rgb(cat.color), window, cx);
                });
            },
        );

        Self {
            calendar,
            title: cx.new(|cx| {
                InputState::new(window, cx).placeholder("标题")
            }),
            all_day: true,
            start_time: cx.new(|cx| {
                DatePickerState::new(window, cx)
                    .time_precision(TimePrecision::Minute)
            }),
            end_time: cx.new(|cx| {
                DatePickerState::new(window, cx)
                    .time_precision(TimePrecision::Minute)
            }),
            color_state,
            desc: cx.new(|cx| {
                TextareaState::new(window, cx)
                    .placeholder("详细描述")
                    .rows(4)
            }),
            _calendar_subscription,
        }
    }

    fn save(&self, app: &App) {
        let db = app.global::<Backend>();
        let calendar_state = self.calendar.read(app);
        let calendar_index =
            calendar_state.selected_index(app).unwrap().row + 1;
        let calendar_name =
            calendar_state.selected_value().unwrap().clone();
        let title = self.title.read(app).content().text().to_string();

        let start_time = self.start_time.read(app).date_time();
        let description =
            self.desc.read(app).content().text().to_string();
        let color = self
            .color_state
            .read(app)
            .value()
            .unwrap()
            .to_rgb()
            .into();
        db.save_event(
            calendar_index as u32,
            calendar_name,
            title,
            self.all_day,
            start_time,
            if self.all_day {
                start_time
            } else {
                self.end_time.read(app).date_time()
            },
            description,
            color,
        );
    }
}

impl Render for NewEventPanel {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .mt_4()
            .gap_4()
            .child(
                h_flex()
                    .justify_between()
                    .gap_5()
                    .child("所属日历")
                    .child(Select::new(&self.calendar)),
            )
            .child(
                h_flex()
                    .justify_between()
                    .gap_5()
                    .child("选择颜色")
                    .child(ColorSelect::new(&self.color_state)),
            )
            .child(Input::new(&self.title))
            .child(
                h_flex().justify_between().child("全天事件").child(
                    Switch::new("all_day_switch")
                        .checked(self.all_day)
                        .on_change(cx.listener(
                            |form, checked, _, _| {
                                form.all_day = *checked;
                            },
                        )),
                ),
            )
            .child(time_select(window, &self.start_time, "开始时间"))
            .when(!self.all_day, |this| {
                this.child(time_select(
                    window,
                    &self.end_time,
                    "结束时间",
                ))
            })
            .child(Textarea::new(&self.desc))
    }
}
