use gpui_kit::{
    Entity, IntoElement, ParentElement, Styled,
    component::{
        TitleBar,
        calendar::Calendar,
        carousel::{
            Carousel, CarouselContent, CarouselItem, CarouselNext,
            CarouselPrevious, CarouselState,
        },
    },
    div, px, relative,
};

use crate::state::State;

pub fn test(state: &mut State) -> impl IntoElement {
    div().size_full().child(
        Carousel::new("month-carousel", state.carousel_state())
            .child(
                CarouselContent::new(state.carousel_state())
                    .child(my_content(
                        0,
                        state.month_view_index(),
                        state.carousel_state(),
                    ))
                    .child(my_content(
                        0,
                        state.month_view_index(),
                        state.carousel_state(),
                    ))
                    .child(my_content(
                        0,
                        state.month_view_index(),
                        state.carousel_state(),
                    )),
            )
            .child(CarouselPrevious::new(state.carousel_state()))
            .child(CarouselNext::new(state.carousel_state())),
    )
}

pub fn my_content(
    i: usize,
    index: usize,
    state: &Entity<CarouselState>,
) -> impl IntoElement {
    CarouselItem::new(format!("project-{}", index), i, &state)
        .child(format!(" 当前是第{}页,当前index:{}", i, index))
        .bg(gpui_kit::red())
}
