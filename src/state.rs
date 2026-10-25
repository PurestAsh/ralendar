use chrono::{Local, NaiveDate};
use gpui_kit::{
    AppContext, Context, Entity, Subscription, Window,
    base::{CalendarEvent, CalendarState, Date},
    component::carousel::{CarouselEvent, CarouselState},
};

use crate::backend::Backend;

pub struct State {
    //按钮控制
    date_switch: [bool; 3],
    //小日历state
    mini_state: Entity<CalendarState>,

    carousel_state: Entity<CarouselState>,

    //月视图当前选择的页面
    month_view_index: usize,

    //subscription
    calendar_subsciption: Subscription,

    //小日历点击后的日期,或者月视图点击的日期.
    select_date: NaiveDate,

    //今天，不变
    today: NaiveDate,

    //Schedule  日程开关
    schedule_toggle: [bool; 3],

    //侧边栏开关
    sidebar_switch: bool,

    backend: Backend,
}

impl State {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let c_state = cx.new(|cx| CalendarState::new(window, cx));
        let carousel_state = cx.new(|cx| {
            CarouselState::new(3)
                .with_selected_index(0)
                .with_looping(true)
        });

        let sub_a = Self::calendar_subscribe(&c_state, cx);
        let sub_b = Self::carousel_subscribe(&carousel_state, cx);

        Self {
            date_switch: [false, true, false],
            calendar_subsciption: Subscription::join(sub_a, sub_b),
            mini_state: c_state,
            carousel_state: carousel_state,
            month_view_index: 0,
            select_date: Local::now().date_naive(),
            today: Local::now().date_naive(),
            schedule_toggle: [true, true, true],
            sidebar_switch: false,
            backend: Backend::new(),
        }
    }

    fn calendar_subscribe(
        e: &Entity<CalendarState>,
        cx: &mut Context<Self>,
    ) -> Subscription {
        cx.subscribe(e, |state, _, event, _| match event {
            CalendarEvent::Selected(date) => match date {
                Date::Single(Some(selected_date)) => {
                    state.mini_select_date(selected_date);
                }
                _ => {
                    println!("Selection cleared");
                }
            },
        })
    }

    fn carousel_subscribe(
        e: &Entity<CarouselState>,
        cx: &mut Context<Self>,
    ) -> Subscription {
        cx.subscribe(e, |state, _, event, cx| {
            let CarouselEvent::Change(index) = event;
            if state.month_view_index != *index {
                state.set_month_view_index(*index);
                cx.notify();
            }
        })
    }

    // 1 0 -> 1  1 1 -> 0 0 -> 0
    // 0 0 -> 0  0 1 -> 1 1 -> 1
    // 0 0 -> 0  0 0 -> 0 0 -> 1
    pub fn switch_mode(&mut self, checkeds: &Vec<bool>) {
        for i in 0..3 {
            self.date_switch[i] ^= checkeds[i];
        }
    }

    pub fn year_check(&self) -> bool {
        self.date_switch[0]
    }

    pub fn month_check(&self) -> bool {
        self.date_switch[1]
    }

    pub fn day_check(&self) -> bool {
        self.date_switch[2]
    }

    pub fn mini_state(&self) -> &Entity<CalendarState> {
        &self.mini_state
    }

    pub fn carousel_state(&self) -> &Entity<CarouselState> {
        &self.carousel_state
    }

    pub fn select_date(&self) -> &NaiveDate {
        &self.select_date
    }

    pub fn change_select_date(&mut self, date: NaiveDate) {
        self.select_date = date;
    }

    pub fn mini_select_date(&mut self, date: &NaiveDate) {
        self.select_date = *date;
    }

    pub fn my_schedule(&self) -> bool {
        self.schedule_toggle[0]
    }

    pub fn cn_schedule(&self) -> bool {
        self.schedule_toggle[1]
    }

    pub fn en_schedule(&self) -> bool {
        self.schedule_toggle[2]
    }

    pub fn toggle_schedule(&mut self, pos: u8) {
        match pos {
            0 => self.schedule_toggle[0] = !self.schedule_toggle[0],
            1 => self.schedule_toggle[1] = !self.schedule_toggle[1],
            2 => self.schedule_toggle[2] = !self.schedule_toggle[2],
            _ => todo!(),
        }

        println!("{:?}", self.schedule_toggle);
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_switch = !self.sidebar_switch;
    }

    pub fn sidebar_switch(&self) -> bool {
        self.sidebar_switch
    }

    pub fn backend(&self) -> &Backend {
        &self.backend
    }

    pub fn is_today(&self) -> bool {
        self.select_date == self.today
    }

    pub fn today(&self) -> &NaiveDate {
        &self.today
    }

    pub fn month_view_index(&self) -> usize {
        self.month_view_index
    }

    pub fn set_month_view_index(&mut self, index: usize) {
        self.month_view_index = index;
    }
}
