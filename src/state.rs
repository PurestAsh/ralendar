use crate::component::mian_content::CalendarState;
use crate::component::side_bar::SideBarState;
pub struct ApplicationState {
    pub sidebar_state: SideBarState,
    pub calendar_state: CalendarState,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            sidebar_state: SideBarState::default(),
            calendar_state: CalendarState::default(),
        }
    }
}
