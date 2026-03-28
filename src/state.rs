use std::time::Duration;

use iced::Animation;
use iced::animation::Easing;

use crate::component::side_bar::SideBarState;
pub struct ApplicationState {
    pub sidebar_state: SideBarState,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            sidebar_state: SideBarState::default(),
        }
    }
}
