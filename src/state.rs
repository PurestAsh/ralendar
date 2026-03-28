use std::time::Duration;

use iced::Animation;

pub struct ApplicationState {
    pub sidebar_state: SideBarState,
}

pub struct SideBarState {
    pub animation: Animation<bool>,
    pub width: f32,
    pub expand: bool,
}

impl Default for SideBarState {
    fn default() -> Self {
        Self {
            animation: Animation::new(true).duration(Duration::from_secs(1)),
            width: 250.,
            expand: true,
        }
    }
}
