use gpui_kit::{Context, StyleRefinement};

use crate::style::{global_style, sidebar};

pub struct State {
    date_switch: [bool; 3],

    global_style: StyleRefinement,
    sidebar: StyleRefinement,
}

impl State {
    pub fn new(cx: &Context<Self>) -> Self {
        Self {
            date_switch: [false, true, false],

            global_style: global_style(cx),
            sidebar: sidebar(cx),
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

    pub fn global_style(&self) -> &StyleRefinement {
        &self.global_style
    }

    pub fn sidebar_style(&self) -> &StyleRefinement {
        &self.sidebar
    }
}
