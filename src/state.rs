use gpui_kit::{
    AppContext, Context, Entity, Window, base::CalendarState,
};

pub struct State {
    //按钮控制
    date_switch: [bool; 3],
    //小日历state
    mini_state: Entity<CalendarState>,
}

impl State {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx| CalendarState::new(window, cx));
        Self {
            date_switch: [false, true, false],
            mini_state: state,
        }
    }

    // 1 0 -> 1  1 1 -> 0 0 -> 0
    // 0 0 -> 0  0 1 -> 1 1 -> 1
    // 0 0 -> 0  0 0 -> 0 0 -> 1
    pub fn switch_date(&mut self, checkeds: &Vec<bool>) {
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
}
