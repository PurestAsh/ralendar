use time::convert::Day;

use crate::backend::Calendar;
use crate::component::calendar::CalendarState;
use crate::component::side_bar::SideBarState;
pub struct ApplicationState {
    pub sidebar_state: SideBarState,
    pub calendar_state: CalendarState,

    //业务state
    pub calendar: Calendar,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            sidebar_state: SideBarState::default(),
            calendar_state: CalendarState::default(),
            calendar: Calendar::new(),
        }
    }
}
