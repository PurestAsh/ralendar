use gpui_kit::Context;

pub struct State {
    date_switch: [bool; 3],
}

impl State {
    pub fn new(_cx: &Context<Self>) -> Self {
        Self {
            date_switch: [false, true, false],
        }
    }

    // 1 0 -> 1  1 1 -> 0 0 -> 0
    // 0 0 -> 0  0 1 -> 1 1 -> 1
    // 0 0 -> 0  0 0 -> 0 0 -> 1
    pub fn switch_date(
        &mut self,
        checkeds: &Vec<bool>,
    ) {
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
}
