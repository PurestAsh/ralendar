use gpui_kit::{
    Entity, IntoElement, ParentElement, Styled,
    component::{
        button::Button,
        carousel::{
            Carousel, CarouselContent, CarouselItem, CarouselState,
        },
        date_picker::DatePicker,
    },
    div,
};

use crate::state::State;

pub fn test(state: &mut State) -> impl IntoElement {
    DatePicker::new(state.start_time_state()).appearance(true)
}
